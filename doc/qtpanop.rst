Qt Panopticon
=============

Synopsis
--------

qtpanopticon [ -a <relocated AVR code> | -n <file> |  -o <panop file> | -p <exe> | -h | -v ]

Description
-----------

Panopticon is a GUI disassembler for various instruct sets and binary file formats. It displays code as a collection of graphs. Panopticon can execute program graphs partially over single values or sets of them. Code and data of a binray can be annonated. These annontations are saved as *panop* files and can be opened later. Panopticon tracks cross references inside code and between code and data structures on heap and stack. Cross referenced locations can be renamed globaly to aide manual analysis.

Flags
-----

All actions specified with flags can be done using the GUI, Most users will want to do the former and should start qtpanopticon without any flags.

-a,--avr <file with relocated AVR code>
    Opens the file, loads it at address zero and starts disassbling AVR code add 0x0000.

-n,--raw <file>
    Opens the file and load it at address zero. No disassbly will take place. Meant for unknown files and files without code.

-o,--open <panop file>
    Loads a previously saved session.

-p,--pe <exe>
    Opens the file and parses it as PE file. Loading and disassbly is done using informations in the PE header.

-h,--help
    Prints a synopsis of all supported flags.

-v,--version
    Prints version information.

Usage
-----

Exit Status
-----------

Examples
--------

Files
-----

See Also
--------
