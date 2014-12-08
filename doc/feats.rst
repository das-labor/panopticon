Features
========

Panopticon is under heavy development and its feature set still very basic.
It's not yet able replace programs like `radare2 <http://radare.org/>`_ and
`IDA Pro <https://www.hex-rays.com/>`_. We are working on it, check back regularly!

What's different about Panopticon is that it is able to understand the code being
analyzed. For Panopticon a line like `add [0x11223344], eax` isn't just a string
that is equal to `0105443322114A`. The application knowns that this instruction
reads the contents of the double word located at address `0x11223344` and adds it to the
value in `eax`, modifying the `CF`, `OF`, `SF`, `ZF`, `AF` and `PF` flags according to
the result.

This allows Panopticon to reason about control flow, memory and register contents.

.. todo::
  Concrete example

The second strength of Panopticon, especially in comparison to its open source
:doc:`alternatives <others>` is the we believe that a excellent graphic UI make a
difference. Panopticon comes with a GUI that exposes all implemented features through
a intuitive, responsive and beautiful Qt 5 application. Panopticon allows direct
manipulation of all elements on screen and tries to make browsing through
thousands of lines of assembly code at least bearable.

Supported Architectures
-----------------------

All analysis and visualisation code is independent of the architecture being analyzed.
This means the each type of analysis can be done on each supported architecture.
Support of a new architecture is just a matter of writing a set of functions translating
the architecture-specific binary patterns into architecture-independent structures.
No deep understanding of the analysis engine is required.

Currently Panopticon is able to disassemble `Atmel AVR <http://www.atmel.com/products/microcontrollers/avr/>`_.
A disassembler for `AMD64 <http://developer.amd.com/resources/documentation-articles/developer-guides-manuals/>`_
(a.k.a. *x64* a.k.a. *x86-64* a.k.a. *Intel 64* a.k.a. *IA-32e*) is work in progress.


Implemented Analysis
--------------------

TODO
