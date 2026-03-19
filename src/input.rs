use rdev::{listen, Event, EventType};
use std::sync::mpsc;

/// Start listening for keyboard events globally.
/// Returns a receiver that emits () on each key press.
pub fn start_listener() -> mpsc::Receiver<()> {
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        listen(move |event: Event| {
            if matches!(event.event_type, EventType::KeyPress(_)) {
                let _ = tx.send(());
            }
        })
        .expect("failed to listen for keyboard events");
    });

    rx
}
