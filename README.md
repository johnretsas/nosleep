# nosleep

A simple command-line tool to prevent your Mac from sleeping
Currently working only on intelx64 macOs. More versions to come.

## Features

- Prevents your Mac's display from sleeping for a specified duration.
- Easily kill the active `nosleep` process.
- Check the remaining time for the active `nosleep` process.

## Installation

**Pre-compiled Binaries**

Download the latest release from the [releases page](https://github.com/johnretsas/nosleep/releases) and move the binary to a directory in your `PATH`.

_(Replace `johnretsas` with your actual GitHub username)_

**From Source**

Ensure you have Rust and Cargo installed.

```bash
git clone https://github.com/johnretsas/nosleep.git
cd nosleep
cargo build --release
```

The binary will be available at `target/release/nosleep`.

## Usage

### Prevent your Mac's display from sleeping:

```bash
nosleep 120
```

This will prevent your Mac's display from sleeping for 120 minutes.

### Kill the active `nosleep` process:

```bash
nosleep -k
```

### Check the remaining time for the active `nosleep` process:

```bash
nosleep -l
```
