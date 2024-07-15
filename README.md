# NES Emulator in Rust

This project is my personal take on creating a NES emulator written in Rust. The goal is to faithfully recreate the experience of playing NES games, leveraging Rust's safety and performance features.

## Features

This emulator is currently a work in progress. The following features are implemented or planned:

- [x] CPU: The central processing unit, responsible for running the game's instructions.
- [ ] PPU (Picture Processing Unit): The graphics processing unit, which will handle rendering the game's visuals.
- [ ] APU (Audio Processing Unit): The audio processing unit, which will handle the game's sound.
- [x] Mapper 0: The simplest memory mapper, often used in early NES games.
- [x] Mapper 1: A more complex memory mapper that allows for more advanced games.
- [x] Mapper 2: Another type of memory mapper used in various NES games.
- [ ] Mapper 3: To be implemented.
- [ ] Mapper 4: To be implemented.
- [ ] Mapper 5: To be implemented.
- [ ] Mapper 6: To be implemented.
- [ ] Mapper 7: To be implemented.

## Usage

### Prerequisites

Before you can build and run the emulator, you need to have the following installed:

- [Rust (nightly)](https://rustup.rs/): The Rust programming language toolchain, specifically the nightly version, which includes the latest features and improvements.
- `custom_error` crate: This Rust crate is used for error handling in the emulator.

### Build

To build the project, navigate to the project directory and run the following command:

```sh
cargo build --release
```

This will compile the emulator in release mode, which optimizes the code for better performance.

### Run

To run the emulator with a specific ROM file, use the following command:

```sh
cargo run --release -- <path-to-rom>
```

Replace `<path-to-rom>` with the path to the ROM file you want to load. This will start the emulator and load the specified ROM.

## References

For more information on the NES architecture and development, you can refer to the following resources:

- [NES Reference Guide](https://wiki.nesdev.com/w/index.php/NES_reference_guide): A comprehensive guide to the NES hardware and software.
- [NES Documentation](https://nesdev.com/NESDoc.pdf): A detailed document covering various aspects of NES development.

These references provide valuable insights and technical details that are essential for developing an NES emulator.

## Credits

This project was inspired by several other NES emulator projects. I would like to acknowledge the following projects for their inspiration and guidance:

- [fogleman/nes](https://github.com/fogleman/nes): An NES emulator written in Go, which served as a great reference for my project.
- [pcwalton/sprocketnes](https://github.com/pcwalton/sprocketnes): Another NES emulator that provided useful insights and ideas for my implementation.

Their work has been instrumental in shaping this project and I am grateful for their contributions to the NES emulator community.
