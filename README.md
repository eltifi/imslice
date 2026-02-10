# imslice

A simple, fast, and cross-platform command-line tool to slice images vertically or horizontally into `n` equal parts.

## Features

- **Vertical Slicing**: Split an image into vertical strips.
- **Horizontal Slicing**: Split an image into horizontal strips.
- **JPEG Support**: Automatically handles oddities like JPEG alpha channels (by converting to RGB).
- **Remainder Handling**: Gracefully handles images that don't divide perfectly by `n`.
- **Supported Formats**: JPEG, PNG, GIF, WebP, AVIF, BMP, ICO, TIFF.
- **Cross-Platform**: Works on Linux, macOS, and Windows.
- **Self-Contained**: Compiles to a single binary with no external dependencies.

## Installation

### From Source
You need [Rust](https://www.rust-lang.org/tools/install) installed.

```bash
git clone https://github.com/eltifi/imslice.git
cd imslice
cargo build --release
```
The binary will be in `target/release/imslice`.

### Binaries
Check the [Releases](https://github.com/eltifi/imslice/releases) page for pre-built binaries for Linux, macOS, and Windows.

## Usage

```bash
imslice <INPUT> -n <N> [-d <DIRECTION>]
```

- `<INPUT>`: Path to the image file (e.g., `sample.jpg`).
- `-n <N>`: Number of slices to create.
- `-d <DIRECTION>`: Direction to slice. `v` for vertical (default), `h` for horizontal.

### Examples

**Slice an image into 3 vertical strips:**
```bash
imslice sample.jpg -n 3
```
*Output: `sample-1.jpg`, `sample-2.jpg`, `sample-3.jpg`*

**Slice an image into 2 horizontal strips:**
```bash
imslice sample.jpg -n 2 -d h
```
*Output: `sample-1.jpg`, `sample-2.jpg`*

## License

MIT
