#!/usr/bin/env python3
"""Convert MIDI file to Typiano JSON song format.

Extracts the melody line (highest notes) from a MIDI file.

Usage:
    python3 midi2typiano.py <input.mid> --id <song-id> --title "Song Title" --composer "Composer"
    python3 midi2typiano.py <input.mid> --id <song-id> --title "Song Title" --composer "Composer" --track 0
"""

import argparse
import json
import sys
from pathlib import Path

import mido

NOTE_NAMES = ['C', 'Db', 'D', 'Eb', 'E', 'F', 'Gb', 'G', 'Ab', 'A', 'Bb', 'B']


def midi_to_note_name(midi_num):
    """Convert MIDI note number to note name (e.g., 60 -> C4)."""
    octave = (midi_num // 12) - 1
    name = NOTE_NAMES[midi_num % 12]
    return f"{name}{octave}"


def extract_melody(mid, track_idx=None, max_notes=200):
    """Extract melody (top voice) from MIDI file.

    Strategy: At each note-on event, if multiple notes overlap,
    keep the highest pitch (typically the melody).
    """
    # Merge all tracks or use specific track
    if track_idx is not None:
        if track_idx >= len(mid.tracks):
            print(f"Warning: track {track_idx} not found, using all tracks")
            merged = mido.merge_tracks(mid.tracks)
        else:
            merged = mid.tracks[track_idx]
    else:
        merged = mido.merge_tracks(mid.tracks)

    notes = []
    active_notes = set()
    last_note = None
    ticks_since_last = 0
    ticks_per_beat = mid.ticks_per_beat

    for msg in merged:
        ticks_since_last += msg.time

        if msg.type == 'note_on' and msg.velocity > 0:
            midi_num = msg.note
            active_notes.add(midi_num)

            # Only register a new melody note if enough time passed
            # (avoid chord notes being registered as separate melody notes)
            min_ticks = ticks_per_beat // 8  # 32nd note threshold

            if ticks_since_last >= min_ticks or last_note is None:
                note_name = midi_to_note_name(midi_num)

                # Filter to playable range (C2-C7)
                octave = midi_num // 12 - 1
                if 2 <= octave <= 7:
                    # Avoid duplicate consecutive notes
                    if note_name != last_note:
                        notes.append(note_name)
                        last_note = note_name
                        ticks_since_last = 0

        elif msg.type == 'note_off' or (msg.type == 'note_on' and msg.velocity == 0):
            active_notes.discard(msg.note)

    return notes[:max_notes]


def extract_melody_top_voice(mid, track_idx=None, max_notes=200):
    """Alternative extraction: collect all note events with timestamps,
    then pick the highest note at each time step."""

    if track_idx is not None and track_idx < len(mid.tracks):
        merged = mid.tracks[track_idx]
    else:
        merged = mido.merge_tracks(mid.tracks)

    # Collect all note-on events with absolute time
    events = []
    abs_time = 0
    for msg in merged:
        abs_time += msg.time
        if msg.type == 'note_on' and msg.velocity > 0:
            events.append((abs_time, msg.note))

    if not events:
        return []

    ticks_per_beat = mid.ticks_per_beat
    # Group notes that happen close together (within 1/16 note)
    group_threshold = ticks_per_beat // 4

    groups = []
    current_group = [events[0]]

    for i in range(1, len(events)):
        if events[i][0] - current_group[0][0] <= group_threshold:
            current_group.append(events[i])
        else:
            groups.append(current_group)
            current_group = [events[i]]
    groups.append(current_group)

    # Pick highest note from each group (melody = top voice)
    melody = []
    last_note = None
    for group in groups:
        highest = max(group, key=lambda x: x[1])
        midi_num = highest[1]
        octave = midi_num // 12 - 1
        if 2 <= octave <= 7:
            note_name = midi_to_note_name(midi_num)
            if note_name != last_note:
                melody.append(note_name)
                last_note = note_name

    return melody[:max_notes]


def print_tracks(mid):
    """Print track info for debugging."""
    print(f"Ticks per beat: {mid.ticks_per_beat}")
    print(f"Number of tracks: {len(mid.tracks)}")
    for i, track in enumerate(mid.tracks):
        note_count = sum(1 for msg in track if msg.type == 'note_on' and msg.velocity > 0)
        name = track.name if track.name else "(unnamed)"
        print(f"  Track {i}: {name} ({note_count} notes)")


def main():
    parser = argparse.ArgumentParser(description='Convert MIDI to Typiano JSON')
    parser.add_argument('input', help='Input MIDI file')
    parser.add_argument('--id', required=True, help='Song ID')
    parser.add_argument('--title', required=True, help='Song title')
    parser.add_argument('--composer', required=True, help='Composer name')
    parser.add_argument('--track', type=int, default=None, help='Track index (default: merge all)')
    parser.add_argument('--max-notes', type=int, default=150, help='Max notes to extract')
    parser.add_argument('--output', '-o', help='Output JSON file (default: songs/<id>.json)')
    parser.add_argument('--info', action='store_true', help='Print track info and exit')
    parser.add_argument('--method', choices=['simple', 'topvoice'], default='topvoice',
                        help='Extraction method')
    parser.add_argument('--min-octave', type=int, default=None, help='Filter out notes below this octave')
    parser.add_argument('--max-octave', type=int, default=None, help='Filter out notes above this octave')

    args = parser.parse_args()

    mid = mido.MidiFile(args.input)

    if args.info:
        print_tracks(mid)
        return

    if args.method == 'topvoice':
        notes = extract_melody_top_voice(mid, args.track, args.max_notes)
    else:
        notes = extract_melody(mid, args.track, args.max_notes)

    # Filter by octave range
    if args.min_octave is not None or args.max_octave is not None:
        filtered = []
        last = None
        for n in notes:
            octave = int(n[-1])
            if args.min_octave is not None and octave < args.min_octave:
                continue
            if args.max_octave is not None and octave > args.max_octave:
                continue
            if n != last:
                filtered.append(n)
                last = n
        notes = filtered[:args.max_notes]

    if not notes:
        print("Error: No notes extracted!", file=sys.stderr)
        sys.exit(1)

    song = {
        "id": args.id,
        "title": args.title,
        "composer": args.composer,
        "notes": notes,
    }

    output = args.output or f"songs/{args.id}.json"
    Path(output).parent.mkdir(parents=True, exist_ok=True)

    with open(output, 'w') as f:
        json.dump(song, f, indent=2)

    unique = len(set(notes))
    octaves = sorted(set(n[-1] for n in notes))
    print(f"Wrote {output}: {len(notes)} notes, {unique} unique, octaves {'-'.join(octaves)}")


if __name__ == '__main__':
    main()
