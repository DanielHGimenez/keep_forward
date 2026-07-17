use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use rdev::{listen, simulate, Event, EventType, Key};

static HOLDING: AtomicBool = AtomicBool::new(false);
// Set by the worker right before it simulates a W press, so the listener can
// tell our own synthetic W from a real user W (which should deactivate).
static SUPPRESS_W: AtomicBool = AtomicBool::new(false);

/// Decide whether a hotkey event should flip the hold state.
/// Only the transition into "satisfied" toggles; staying satisfied is ignored.
fn should_toggle(satisfied: bool, already_active: bool) -> bool {
    satisfied && !already_active
}

/// True when every key of the chosen combo is currently held.
fn combo_satisfied(combo: &HashSet<Key>, held: &HashSet<Key>) -> bool {
    combo.iter().all(|k| held.contains(k))
}

fn main() {
    println!("keep_forward: hold up to 3 keys together then release to set your hotkey.");
    println!("  press it to toggle holding W, Ctrl+C to quit.");
    println!("  macOS: grant Accessibility permission. Linux: X11 only (not Wayland).");

    // Listener thread: capture the hotkey combo, then flip HOLDING on it. Kept
    // separate from simulate() to avoid deadlocking inside rdev's listen callback.
    thread::spawn(|| {
        let mut held: HashSet<Key> = HashSet::new();
        let mut combo: Option<HashSet<Key>> = None;
        let mut capturing: HashSet<Key> = HashSet::new();
        let mut active = false;
        let callback = move |event: Event| match event.event_type {
            EventType::KeyPress(k) => {
                held.insert(k);
                // A real W press deactivates. Skip the synthetic W we simulate.
                if k == Key::KeyW && combo.is_some() {
                    if SUPPRESS_W.swap(false, Ordering::Relaxed) {
                        // our own simulated press; ignore
                    } else if HOLDING.load(Ordering::Relaxed) {
                        HOLDING.store(false, Ordering::Relaxed);
                        active = false;
                        println!("holding W: false (W pressed)");
                    }
                    return;
                }
                match &combo {
                    None => {
                        if capturing.len() < 3 {
                            capturing.insert(k);
                        }
                    }
                    Some(c) => {
                        if should_toggle(combo_satisfied(c, &held), active) {
                            let now = !HOLDING.load(Ordering::Relaxed);
                            HOLDING.store(now, Ordering::Relaxed);
                            println!("holding W: {now}");
                            active = true;
                        }
                    }
                }
            }
            EventType::KeyRelease(k) => {
                held.remove(&k);
                match &combo {
                    None => {
                        if held.is_empty() && !capturing.is_empty() {
                            let c = std::mem::take(&mut capturing);
                            println!("hotkey set to {c:?} pressed in any order. Press it to toggle holding W.");
                            combo = Some(c);
                        }
                    }
                    Some(c) => {
                        if !combo_satisfied(c, &held) {
                            active = false;
                        }
                    }
                }
            }
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
            if want {
                // ponytail: flag-before-simulate can race a real W in the same
                // instant; fine for a single edge. Tighten only if it misfires.
                SUPPRESS_W.store(true, Ordering::Relaxed);
            }
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
    use super::{combo_satisfied, should_toggle};
    use rdev::Key;
    use std::collections::HashSet;

    #[test]
    fn debounce() {
        assert!(should_toggle(true, false)); // first satisfied toggles
        assert!(!should_toggle(true, true)); // still held: ignored
        assert!(!should_toggle(false, false)); // not satisfied: not a toggle
        assert!(should_toggle(true, false)); // satisfied again after release
    }

    #[test]
    fn combo_subset_match() {
        let combo = HashSet::from([Key::ControlLeft, Key::F8]);
        let with_extra = HashSet::from([Key::ControlLeft, Key::F8, Key::ShiftLeft]);
        let partial = HashSet::from([Key::ControlLeft]);
        assert!(combo_satisfied(&combo, &with_extra)); // extra held keys don't block
        assert!(!combo_satisfied(&combo, &partial)); // missing a combo key
    }
}
