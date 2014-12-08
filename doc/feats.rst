Features
========

Panopticon is under heavy development and its feature set still very basic.
It's not yet able replace programs like `radare2 <http://radare.org/>`_ and
`IDA Pro <https://www.hex-rays.com/>`_. We are working on it, check back regularly!

What's different about Panopticon is that it is able to understand the code being
analyzed. For Panopticon a line like ``add [0x11223344], eax`` isn't just a string
that is equal to the byte sequence ``0105443322114A``. The application knowns that
this instruction reads the contents of the double word located at address
``0x11223344`` and adds it to the value in ``eax`` and modifies the ``CF``, ``OF``,
``SF``, ``ZF``, ``AF`` and ``PF`` flags according to the result.

This allows Panopticon to reason about control flow, memory and register contents.

.. todo::
  Concrete example

The second strength of Panopticon -- especially in comparison to its open source
:doc:`alternatives <others>` -- is that we believe that an excellent graphical UI makes a
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

Currently Panopticon is able to disassemble
`Atmel AVR <http://www.atmel.com/products/microcontrollers/avr/>`_. A disassembler
for `AMD64 <http://developer.amd.com/resources/documentation-articles/developer-guides-manuals/>`_
(a.k.a. *x64* a.k.a. *x86-64* a.k.a. *Intel 64* a.k.a. *IA-32e*) is work in progress.


Implemented Analysis
--------------------

Panopticon implements classic data flow analysis as well as more sofisticated
`Abstract Interpretation <AI>`_-based algorithms that can partially execute
code. Analysis is always done in background and on-demand, no need to trigger
it manually using the UI.

Data Flow Graph
~~~~~~~~~~~~~~~

As part of the disassembly step, the assembler code is transformed into an
intermediate language. This language uses `Static Single Assignment <SSA>`_ Form
which makes data flow and data dependencies explicit.

.. todo:: Data flow example.

Dominator Tree
~~~~~~~~~~~~~~

The Dominator tree of each procedure is computed as part of the SSA transformation.
This tree can be displayed in the UI.

Execution Over Sets
~~~~~~~~~~~~~~~~~~~

Using the `Abstract Interpretation <AI>`_ Framework implemented in Panopticon,
code can be executed locally over sets of values. This way Panopticon can resolve
indirect jumps and calls while disassembling.

.. todo:: Indirect jump example.
