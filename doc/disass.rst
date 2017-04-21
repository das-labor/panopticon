Disassembler
============

A disassembler in Panopticon is responsible to translate a sequence of tokens
into mnemonics. A token is a fixed width byte sequence. The width depends on the
instruction set architection and is the shortest possible machine code
instruction (on IA32 this would be 1 byte, on ARM 4 bytes). A mnemonic includes
the syntax of the machine code instruction, is semantics in PIL and a collection
of locations the CPU will look for the next instruction to execute. For each
supported instruction set architecture a seperate disassembler needs to be
implemented. All implementations are specializations of the :cpp:class:`disassembler\<T>`
template. The type parameter identifies the instruction set. When machine code
needs to be disassembled, a new instance of :cpp:class:`disassembler\<T>` is allocated and its
:cpp:func:`match()` method is repeantly called. Each call returns either a mnemonic and a
set of new locations of an error. Disassembly is finished when no new locations
are left.

The :cpp:class:`disassembler\<T>` type template provides fuctions to make disassembly easier.
The programmer only need to write one decode function for each instruction in
the instruction set. This decode function translates the byte representation
into one or more mnemonic instances with instruction name, operand count and
instruction semantics expressed as a PIL instruction sequence. Each decode
functions is paired with a token pattern. The disassembler instance will look
for this pattern and call the decode function for each match. The menmonic
instances allocated in the decode function are assembled into a program.

Token Patterns
--------------

The token pattern is a string that defines sequence on bits to look for. Each
bit in a pattern is either ``0``, ``1`` or ``.`` when we accept both. The pattern
``10001001`` matches the byte ``0x89``, the pattern ``11.100.0`` matches ``0xd0``
(``11010000``), ``0xd2`` (``11010010``), ``0xf0`` (``11110000``) and ``0xf2`` (``11110010``). Pattern must
have one pattern character for each bit in the token. Patters allow named groups
of bits so called capture groups. These start with a character except ``0``, ``1``,
``.`` and `` `` (space), followed by a ``@``, followed by a pattern. The capture group
extend until the next space character or the end of the pattern string. The
pattern ``10 a@110 011`` has the capture group named `a` that is always equal to
``0x6`` (``110``). The pattern ``001 a@.....`` matches all tokens larger than or equal to
``0x20``, the least significant 5 bits form the capture group `a`. When a pattern is
paired with a decode function in the disassembler the function receives the
contents of capture groups a an argument.

Example
-------

An example pair of decoder function token pattern for the AVR ``pop`` instruction
looks like this:

.. code-block:: c++

   struct avr_tag {};

   disassembler<avr_tag> main;

   main | "1001000 d@..... 1111" = [](const sem_state<avr_tag>& st)
   {
      variable op("r" + std::to_string(st.capture_groups["d"]), 8);
      state.mnemonic(2, "pop", "{8}", op);
   };

Pairing patterns with decode functions is done using the following syntax:

.. code-block:: ebnf

   DisassemblerInstance "|"
       (TokenPattern | DisassemblerInstance)
       { "|" (TokenPattern | DisassemblerInstance) }
       "=" SemanticFunction

See Also
--------

.. cpp:class:: po::disassembler<T>

  Base class for all disassemblers.

Todo: mnemonic and codegen, pattern -> func, subdecoder


