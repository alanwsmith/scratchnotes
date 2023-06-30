# speednotes

Quick notes from the command line

## Overview

speednotes is a CRUD app for notes on the 
command line. It's designed to be simple and
quick. 

## Usage

### List notes

```bash
s
```

### Read/Edit a note 

```bash
s notename 
```

### Create a note 

```bash
s -n notename 
```

### Delte a note 

```bash
s -d notename 
```

### View help 

```bash
s -h
```

## Current Status

- [x] Make `~/.speednotes/notes` if it doesn't exist

- [x] Store files in `~/.speednotes/notes`

- [x] List out all files if the `s` command is run with no arguments

- [x] Sort file listing

- [x] Open files when the are called by name

- [x] Setup `-n notename` to make new files

- [x] Setup `-d notename` to remove files

- [x] Show a message if there are no notes in the archive

- [x] Setup `-h` to display help (already built in)

- [ ] If a file doesn't exist, as if you want to create it

- [ ] Autocomplete file names from in the arguments

- [ ] Allow for spaces in file names



## Installation

This is currently in the "Works on my machine" 
phase. If you clone the repo down on mac and run 
the install with this it's got a good chance of 
adding it to your path so you can use it:


```
cargo install --path .
```

