# image-organiser

This is a simple tool I'm writing mainly for my own purposes. It organises files from my camera's SD card into folders based on the date the photo was taken. It also renames the files to a more sensible format.
Initially I used Swift because that's what I'm most familiar with, but I'm now rewriting it in Rust since it's all the rage now and I've been meaning to try it for a while.

## Usage

```bash
# To copy the matched files to the TARGET_DIRECTORY
image-organiser copy <PATTERN> <TARGET_DIRECTORY> [SOURCE_DIRECTORY]

# To move the matched files to the TARGET_DIRECTORY
image-organiser move <PATTERN> <TARGET_DIRECTORY> [SOURCE_DIRECTORY]
```

The parameters are the same for both the `copy` and `move` commands.

- `PATTERN` parameter is a glob pattern that matches the files you want to organise.
- `TARGET_DIRECTORY` is the directory you want the files to be copied/moved to.
- `SOURCE_DIRECTORY` is the directory you want to search for files in. If it's not specified, the current directory is used.

## Progress

About 60% done. I've got the basic functionality working, but there are a few things I want to add before I'm happy with it.
