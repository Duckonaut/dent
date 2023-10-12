# Dent
> Duckonaut's Extensible Notation for Things.

## About
Dent is an extensible format for storing slightly-advanced data.

It's basic notation is simplistic to be human friendly, but allows
for some basic operations at file parse time, such as:
- importing other `.dent` files as parts of your file,
- merging split-up dictionaries or lists.

Many other operations you may need can be added with closures passed
to the `Dent` struct.

## Features
- Simple syntax
- Integers, floats, bools, strings
- Dictionaries
- Lists
- Comments
- Extensible functions

## Examples
```
# file: examples/dent/dict.dent

# Mario
{
    name: Mario
    skills: [
        jumps
        grows
    ]
    age: 35
    alive: true
}

# ...

# another file:

# Define a list of characters
[
    @import "examples/dent/dict.dent"
    {
        name: Link
        skills: [
            swordfighting
            reincarnation
        ]
        age: 23
        alive: true
    }
]
```
