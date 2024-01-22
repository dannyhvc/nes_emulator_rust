# NES Emulator in Rust

My personal take on a new emulator written in Rust.

## Features

-----

- [x] CPU
- [ ] PPU
- [ ] APU
- [x] Mapper 0
- [x] Mapper 1
- [x] Mapper 2
- [ ] Mapper 3
- [ ] Mapper 4
- [ ] Mapper 5
- [ ] Mapper 6
- [ ] Mapper 7

## Usage

------

### Prerequisites

- Rust (nightly)
- custom_erro

### Build

cargo build --release

### Run

cargo run --release –`<path-to-rom>`

## References

-----

- <https://wiki.nesdev.com/w/index.php/NES_reference_guide>
- <https://nesdev.com/NESDoc.pdf>

## Credits

-----

This project was inspired by the following projects:

- <https://github.com/fogleman/nes>
- <https://github.com/pcwalton/sprocketnes>
