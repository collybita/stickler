# Stickler

Stickler is a small cli program to handle collectable numbered sticker collections.

## Planned commands

The following command options are yet to be implemented, and prone to change.

How to launch
```
stickler # launches app with empty collection
stickler /path/to/collection # loads existing collection or creates an empty one if it does not already exist
```

Usage
```
list # prints collection content (collected, missing, tradeable, trades)
add [NUMBERS] # adds stickers to the collection (numbers separated by spaces)
remove [NUMBERS] # removes stickers from collection
trade prep|unprep|make # prepares, unprepares or makes a trade