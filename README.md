<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/8/87/Arduino_Logo.svg/1280px-Arduino_Logo.svg.png" width="120" />
  </div>
  <h1 align="center">Arduino Projects</h1>
  <h4 align="center">
    Collection of projects done Arduino with Rust
  </h4>
</div>

## Setup

1. Make sure XCode Developer Tools are installed

```bash
xcode-select --install
```

2. Install AVR dependencies

```bash
brew tap osx-cross/avr
brew install avr-gcc arvdude
```

3. Install Ravedude

```rust
cargo install ravedude
```

4. Make sure `avr-specs` has a JSON file for your microcontroller, if missing,
add it from https://github.com/Rahix/avr-hal/tree/main/avr-specs.

5. Configure `.cargo/config.toml` to point the `target` to your microcontroller spec.

6. Find your device's port using `/bin/ravedude_port.sh`

## Running Locally

Each crate has a README with a diagram, motivation and example command.

## License

MIT Licensed
