# keep_forward

Auto-walk forward in games without holding **W**. Bind your own hotkey, press
it to toggle **W** held down, press **W** to stop.

**Website:** https://danielhgimenez.github.io/keep_forward/

## How it works

1. **Run it** from a terminal — no install, no config.
2. **Set your hotkey**: hold up to three keys together and release; that combo
   is bound (matched in any order).
3. **Press to toggle**: tap your hotkey and W stays held down. Tap again to release.
4. **Tap W** yourself any time to turn holding off.

On Windows it injects the `W` hardware scancode (`0x11`) via `SendInput`, so
DirectInput / Raw Input games actually register the key rather than ignoring
virtual-key-only events. On macOS and Linux it uses `rdev`'s simulate.

## Platforms

| Platform | Notes |
|----------|-------|
| Windows  | x86_64 (scancode injection) |
| macOS    | Apple Silicon & Intel — grant Accessibility permission |
| Linux    | x86_64, X11 only (not Wayland) |

## Install

Download a prebuilt binary from the
[latest release](https://github.com/DanielHGimenez/keep_forward/releases/latest),
or build from source:

```sh
make build        # cargo build --release
make windows      # cross-compile to Windows (needs the x86_64-pc-windows-gnu target)
make test
```

Run the binary, set your hotkey, and toggle. `Ctrl+C` to quit.
