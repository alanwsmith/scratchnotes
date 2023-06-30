# h 

(for your personal help files)

## Overview

This goal of this project is to make a simple command
called `h`` that runs from the command line. It displays
the content of text files. The idea is to make 
little cheat sheets and get to them quickly.


## Usage

```
h whatever
```

And to create a new file (work in progress)

```
h -n whatever
```


which will output the contents of:

```
~/.h-files/whatever.txt
```


## Current Status


[x] Store files in `~/.h-files`` 

[x] Show a file if an argument with the same name
is passed (e.g. `h whatever`` displays 
`~/.h-files/whatever.txt``)

[x] List out all files if the `h`` command is
run with no arguments

[x] make the ~/.h-files directory if it doesn't 
already exist

[x] Sort direcotry listing of existing files

[] Setup `--edit whatever` to open the file in 
the default editor

[] Setup `--new whatever` to make new files

[] Setup `--delete whatever` to remove files

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


