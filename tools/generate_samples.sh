#!/bin/bash
# Generate piano-like WAV samples using ffmpeg
# Piano tone = fundamental + harmonics with per-harmonic decay

OUTDIR="$(dirname "$0")/../assets/samples"
mkdir -p "$OUTDIR"

declare -A FREQS
FREQS=(
    ["C2"]=65.41  ["Db2"]=69.30  ["D2"]=73.42  ["Eb2"]=77.78
    ["E2"]=82.41  ["F2"]=87.31  ["Gb2"]=92.50  ["G2"]=98.00
    ["Ab2"]=103.83 ["A2"]=110.00 ["Bb2"]=116.54 ["B2"]=123.47
    ["C3"]=130.81 ["Db3"]=138.59 ["D3"]=146.83 ["Eb3"]=155.56
    ["E3"]=164.81 ["F3"]=174.61 ["Gb3"]=185.00 ["G3"]=196.00
    ["Ab3"]=207.65 ["A3"]=220.00 ["Bb3"]=233.08 ["B3"]=246.94
    ["C4"]=261.63 ["Db4"]=277.18 ["D4"]=293.66 ["Eb4"]=311.13
    ["E4"]=329.63 ["F4"]=349.23 ["Gb4"]=369.99 ["G4"]=392.00
    ["Ab4"]=415.30 ["A4"]=440.00 ["Bb4"]=466.16 ["B4"]=493.88
    ["C5"]=523.25 ["Db5"]=554.37 ["D5"]=587.33 ["Eb5"]=622.25
    ["E5"]=659.26 ["F5"]=698.46 ["Gb5"]=739.99 ["G5"]=783.99
    ["Ab5"]=830.61 ["A5"]=880.00 ["Bb5"]=932.33 ["B5"]=987.77
    ["C6"]=1046.50 ["Db6"]=1108.73 ["D6"]=1174.66 ["Eb6"]=1244.51
    ["E6"]=1318.51 ["F6"]=1396.91 ["Gb6"]=1479.98 ["G6"]=1567.98
    ["Ab6"]=1661.22 ["A6"]=1760.00 ["Bb6"]=1864.66 ["B6"]=1975.53
    ["C7"]=2093.00
)

rm -f "$OUTDIR"/*.wav

for note in "${!FREQS[@]}"; do
    freq=${FREQS[$note]}
    outfile="$OUTDIR/${note}.wav"

    echo "Generating $note (${freq}Hz)..."

    freq2=$(printf "%.2f" "$(echo "$freq * 2" | bc -l)")
    freq3=$(printf "%.2f" "$(echo "$freq * 3" | bc -l)")
    freq4=$(printf "%.2f" "$(echo "$freq * 4" | bc -l)")
    freq5=$(printf "%.2f" "$(echo "$freq * 5" | bc -l)")

    # Higher notes = shorter sustain
    if (( $(echo "$freq < 200" | bc -l) )); then
        dur=3.0; fd=2.5
    elif (( $(echo "$freq < 500" | bc -l) )); then
        dur=2.0; fd=1.7
    elif (( $(echo "$freq < 1000" | bc -l) )); then
        dur=1.5; fd=1.3
    else
        dur=1.0; fd=0.9
    fi

    fd2=$(printf "%.2f" "$(echo "$fd * 0.7" | bc -l)")
    fd3=$(printf "%.2f" "$(echo "$fd * 0.5" | bc -l)")
    fd4=$(printf "%.2f" "$(echo "$fd * 0.35" | bc -l)")
    fd5=$(printf "%.2f" "$(echo "$fd * 0.25" | bc -l)")

    ffmpeg -y -loglevel error \
        -f lavfi -i "sine=frequency=${freq}:duration=${dur}" \
        -f lavfi -i "sine=frequency=${freq2}:duration=${dur}" \
        -f lavfi -i "sine=frequency=${freq3}:duration=${dur}" \
        -f lavfi -i "sine=frequency=${freq4}:duration=${dur}" \
        -f lavfi -i "sine=frequency=${freq5}:duration=${dur}" \
        -filter_complex "\
            [0:a]volume=0.50,afade=t=out:st=0.3:d=${fd}[h1]; \
            [1:a]volume=0.22,afade=t=out:st=0.1:d=${fd2}[h2]; \
            [2:a]volume=0.12,afade=t=out:st=0.05:d=${fd3}[h3]; \
            [3:a]volume=0.06,afade=t=out:st=0.02:d=${fd4}[h4]; \
            [4:a]volume=0.03,afade=t=out:st=0.01:d=${fd5}[h5]; \
            [h1][h2][h3][h4][h5]amix=inputs=5:duration=longest:normalize=0, \
            afade=t=in:d=0.005, \
            volume=2.5 \
        " \
        -ar 44100 -ac 1 -sample_fmt s16 \
        "$outfile"
done

echo ""
echo "Done! Generated $(ls "$OUTDIR"/*.wav 2>/dev/null | wc -l) samples."
