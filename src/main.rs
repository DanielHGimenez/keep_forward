use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use rdev::{listen, simulate, Event, EventType, Key};

static HOLDING: AtomicBool = AtomicBool::new(false);

/// Decide whether an F8 event should flip the hold state.
/// Only the first press toggles; OS key-repeat (while already down) is ignored.
fn should_toggle(is_f8_press: bool, f8_already_down: bool) -> bool {
    is_f8_press && !f8_already_down
}

fn main() {
    println!("keep_forward: press F8 to toggle holding W, Ctrl+C to quit.");
    println!("  macOS: grant Accessibility permission. Linux: X11 only (not Wayland).");

    // Listener thread: watch F8, flip HOLDING. Kept separate from simulate() to
    // avoid deadlocking inside rdev's listen callback.
    thread::spawn(|| {
        let mut f8_down = false;
        let callback = move |event: Event| match event.event_type {
            EventType::KeyPress(Key::F8) => {
                if should_toggle(true, f8_down) {
                    let now = !HOLDING.load(Ordering::Relaxed);
                    HOLDING.store(now, Ordering::Relaxed);
                    println!("holding W: {now}");
                }
                f8_down = true;
            }
            EventType::KeyRelease(Key::F8) => f8_down = false,
            _ => {}
        };
        if let Err(e) = listen(callback) {
            eprintln!("input listen failed: {e:?}");
        }
    });

    // Worker loop: act on HOLDING edges. simulate() lives here, off the listen thread.
    let mut pressed = false;
    loop {
        let want = HOLDING.load(Ordering::Relaxed);
        if want != pressed {
            let ev = if want {
                EventType::KeyPress(Key::KeyW)
            } else {
                EventType::KeyRelease(Key::KeyW)
            };
            if let Err(e) = simulate(&ev) {
                eprintln!("simulate failed: {e:?}");
            }
            pressed = want;
        }
        thread::sleep(Duration::from_millis(50));
    }
}

#[cfg(test)]
mod tests {
    use super::should_toggle;

    #[test]
    fn debounce() {
        assert!(should_toggle(true, false)); // first press toggles
        assert!(!should_toggle(true, true)); // key-repeat while down: ignored
        assert!(!should_toggle(false, false)); // release: not a toggle
        assert!(should_toggle(true, false)); // press after release toggles again
    }
}
