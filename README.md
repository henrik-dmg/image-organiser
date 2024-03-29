# image-organiser

This is a simple tool I'm writing mainly for my own purposes. It organises files from my camera's SD card into folders based on the date the photo was taken. It also renames the files to a more sensible format.
Basically a Rust version of my Swift tool [file-organiser](https://github.com/henrik-dmg/FileOrganiser). If you're on macOS, I recommend using that instead because I have little to no idea what I'm doing with Rust :D

## Installation

### Cargo

Using `cargo` is the simplest way to install, simply run `cargo install image-organiser`

### Manual

- Clone the repo using `git clone https://github.com/henrik-dmg/image-organiser.git`
- Change to the repo's directory `cd image-organiser`
- Build using `cargo build`

## Usage

```bash
# To copy the matched files to the TARGET_DIRECTORY
image-organiser copy <PATTERN> <TARGET_DIRECTORY> [SOURCE_DIRECTORY] [STRATEGY]

# To move the matched files to the TARGET_DIRECTORY
image-organiser move <PATTERN> <TARGET_DIRECTORY> [SOURCE_DIRECTORY] [STRATEGY]
```

The parameters are the same for both the `copy` and `move` commands.

- `PATTERN` parameter is a glob pattern that matches the files you want to organise.
- `TARGET_DIRECTORY` is the directory you want the files to be copied/moved to.
- `SOURCE_DIRECTORY` is the directory you want to search for files in. If it's not specified, the current directory is used.
- `STRATEGY` is the strategy to use when copying/moving files. If it's not specified, the default `month` is used.

Example: `image-organiser copy "**/*.jpg" /Volumes/MySDCard ~/Pictures week`

## Progress

About 80% done. I've got the basic functionality working, but there are a few things I want to add before I'm happy with it.
