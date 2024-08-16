
# Tim

Command line text editor like vim. But tim.

Let's you open a file in your terminal to edit files with word wrap and a undo functionality.

Tim also includes a file explorer to quickly select the correct file.

# Commands
```
Command line text editor like vim. But tim.

Usage: tim <FILE_PATH> [OPTIONS]

Options:
    -c, --create        Creates but doesn't open file
    -d, --delete        Deletes file
    -r, --rename [NAME] Renames file to [NAME] or user inputted
    -b, --dark          White on black
    -l, --light         Black on white

Usage: tim [OPTIONS]

Options:
    -f, --files         Opens a file explorer to pick a file to open
    -h, --help          Shows commands
    -k, --keybinds      Shows keybinds/controls
```

# Examples
```
tim foo.txt --create            // Creates foo.txt
tim foo.txt --rename bar.txt    // Renames foo.txt to bar.txt
tim bar.txt --delete            // Deletes bar.txt

tim foo.txt -- create           // Creates foo.txt
tim foo.txt -- rename           // Renames foo.txt to bar.txt
Enter a new name for the file:
bar.txt

tim bar.txt -- delete           // Deletes bar.txt
```

Pretty self explanatory.

# Controls

```
Text Editor:
    Esc, End, Delete, Ctrl-S => Exit
    Arrow Keys => Move Cursor
    Ctrl-Z => Undo

File Explorer:
    Esc, End, Delete, Ctrl-S => Exit
    Arrow Keys => Move Cursor
    Enter, Space => Select
    Backspace => Parent Directory
```
