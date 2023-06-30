# speednotes

Quick notes from the command line

## Overview

speednotes is a CRUD app for notes on the 
command line. It's designed to be simple and
quick. 

## Usage

speednotes does five things:

### List notes

```bash
s
```

### Create a note 

```bash
s -n notename 
```

### Read a note 

```bash
s notename 
```

### Edit a note 

```bash
s -e notename 
```

### Delte a note 

```bash
s -d notename 
```


## Current Status


[] Store files in `~/.speednotes/notes`` 

[] Show a file if an argument with the same name
is passed (e.g. `s notename`` displays 
`~/.speedfiles/notes/notename.txt``)

[] List out all files if the `speednotes`` command is
run with no arguments

[] make the ~/.speednotes directory if it doesn't 
already exist

[] Sort direcotry listing of existing files

[] Setup `-e notename` to open the file in 
the default editor

[] Setup `-n notename` to make new files

[] Setup `-d notename` to remove files

[] Autocomplete file names from in the arguments 

[] Allow for typing in numbers to open a file
based on the order of the listing

[] Set the spacer lines equal to the longest
line of text or the width of the terminal 
(whichever is shortest)


## Installation

This is currently in the "Works on my machine" 
phase. If you clone the repo down on mac and run 
the install with this it's got a good chance of 
adding it to your path so you can use it:


```
cargo install --path .
```


