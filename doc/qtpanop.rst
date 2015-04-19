Qt Panopticon
=============

Synopsis
--------

qtpanopticon [ -a <relocated AVR code> | -n <file> |  -o <panop file> | -p <exe> | -h | -v ]

Description
-----------

Panopticon is a GUI disassembler for various instruct sets and binary file formats. It displays code as a collection of graphs. Panopticon can execute program graphs partially over single values or sets of them. Code and data of a binary can be annotated. These annotations are saved as *panop* files and can be opened later. Panopticon tracks cross references inside code and between code and data structures on heap and stack. Cross referenced locations can be renamed globally to aide manual analysis.

Flags
-----

All actions specified with flags can be done using the GUI, Most users will want to do the former and should start qtpanopticon without any flags.

-a,--avr <file with relocated AVR code>
    Opens the file, loads it at address zero and starts disassembling AVR code at 0x0000.

-A,--avr-mcu <MCU name>
    Assume another MCU than ATmega88 when disassembling.

-n,--raw <file>
    Opens the file and load it at address zero. No disassembly will take place. Meant for unknown files and files without code.

-o,--open <panop file>
    Loads a previously saved session.

-p,--pe <exe>
    Opens the file and parses it as PE file. Loading and disassembly is done using informations in the PE header.

-h,--help
    Prints a synopsis of all supported flags.

-v,--version
    Prints version information.

Usage
-----

If no flags are given qtpanopticon displays a prompt that asks the user to select a file to disassemble or to start with a completely empty session. If the -a, -n, -o or -p flags is given this prompt is skipped.

The workspace view displayed after selecting a file to work on (either using flags or the GUI prompt) lists all procedures found in the binary. Clicking these selects them in the view. The workspace can be toggled between the Graph View and the Linear View. The Graph View shows the selected procedures control flow graph. Basic blocks can be moved around by dragging them. Hovering over a basic block highlights in- and outgoing control flow edges. The Linear View shows the raw contents of the file as a hexdump with disassembled parts interleaved. Selecting a procedure scrolls the Linear View to the start of the entry basic block. Control flow edges between basic blocks are drawn on the left.

See Also
--------

https://panopticon.re/
