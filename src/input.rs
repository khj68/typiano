use rdev::{listen, Event, EventType, Key};
use std::sync::mpsc;

/// Start listening for keyboard events globally.
/// Returns a receiver that emits the pressed Key on each key press.
pub fn start_listener() -> mpsc::Receiver<Key> {
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        listen(move |event: Event| {
            if let EventType::KeyPress(key) = event.event_type {
                let _ = tx.send(key);
            }
        })
        .expect("failed to listen for keyboard events");
    });

    rx
}
