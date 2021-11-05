# count-files

## Introduction
count-files is a small CLI tool to count files and directories within a base directory, like what you would normally see when right-clicking a folder in your file manager.
It has the ability to recursively count files in each directory, as well as differentiate between normal and hidden files and directories.

## Flags 
| Short | Takes value | Function |
| ----- | ----------- | -------- |
| -b | Yes | Takes a path to a base directory, otherwise uses parent directory |
| -r | No | Enables recursion, turned off by default |
| -i | No | Includes hidden files and directories, turned off by default |
| -m | Yes | Max depth of subdirectories, set to 1000 by default |
| -s | Yes | Sorting algorithm for displaying list of directories, "ascending", "descending", "alphabetical" and "reverse-alphabetical" available, with "ascending" set as default |
| -l | Yes | Limit the number of directories printed if displaying directories is enabled, default set to 1000 |
| -d | No | If enabled will display directories and the number of files in them |
