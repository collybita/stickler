# Stickler

Stickler is a small cli program to handle collectable numbered sticker collections.

## Planned commands

The following command options are yet to be implemented, and prone to change.

```
stickler new [COLLECTION]                       # creates new collection
stickler load [COLLECTION]                      # loads collection
stickler delete [COLLECTION]                    # deletes collection
stickler list                                   # lists collected, missing, and tradeable stickers
stickler add [NUMBERS]                          # adds stickers to the collection (numbers separated by spaces)
stickler remove [NUMBERS]                       # removes stickers from the collection
stickler trade prep [TRADE] [NUMBERS]:[NUMBERS] # creates a trade with a given name and numbers
stickler trade revert [TRADE]                   # reverts a created trade
stickler trade make [TRADE]                     # makes a trade
stickler trade list                             # lists all trades
stickler trade list [TRADE]                     # lists items in a given trade
```