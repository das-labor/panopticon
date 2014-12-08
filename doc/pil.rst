Intermediate Language
=====================

Panopticon uses a intermediate language to model mnemonic semantics.

Conventional disassembler translate machine code from its binary representaion to into a list of mnemonics similar to the format assemblers accept. The only knowlegde the disassembler has of the opcode is its textual form (for example "mov") and the number and type (constant vs. register) of operands. These informations are purly "syntactic" – they are only about opcode shape. Advanced disassembler like distorm or IDA Pro add limited semantic information to an opcode like whenever it's a jump or how executing it effects the stack pointer. This ultimatly limits the scope and acurcy of analysis a disassembler can do.

Reverse engineering is about understanding code. Most of the time the analyst interprets assembler instructions by "executing" them in his or her head. Good reverse engeineers are those who can do this faster and more aquratly than others. In order to help human analysts in this labourus task the disassembler needs to understand the semantics of each mnemonic.

Panopticon uses a simple and well defined programming language (called PIL) to model the semantics of mnemonics in a machine readable manner. This intermediate languages is emitted by the disassembler part of Panopticon and used by all analysis algorithms. This way the analysis implementation is decoupled from the details of the instruction set.

Basic structure
---------------

A PIL program modeling the AVR "adc" instruction looks as this:

.. math::
  a \rightarrow b

Each PIL program is a seqence of assignemnts. The left side is either a variable or a memory reference. The right side is a single operation with one or more arguments. PIL has two types of values, integers and booleans. PIL is strongly typed, operations that work on integers will not accept boolean values and vise versa. Conversion between those two types must be done explicitly. Integers allow simple linear arithmetic and comparison:

- add
- sub
- ...

Booleans support first order logic and conversion to integers:

- and
- xor
- ...

Memory in PIL programs is modeled as an array of memory cells. These arrays are called memory banks and have unique names used for identification. The cells are numbered in acending order starting at 0. This nmber is the offset of the cell. If mutiple cells are accessed at once, cells can either be interpreted in Little Endian (torwards lower offsets) or Big Endian (torwards higher offsets). In conclusion, a read- and writable memory reference consist of the memory bank name, the offset of the first cell to be read/written, the number of cells to work on and whenever Big or Little Endian byte ordering should be honored.

.. code-block:: c++

  f = a[0x1,1,little-endian]
  b[a,3,big-endian] = 0x1

Aside from boolean and integer constants, variables and memory references PIL programs can use undefined values. Setting a variable or memory cell to "undefined" tells the analysis engine that the operation either has no result or that this value can not be by determined by the disassembler. A example for the first case is the "call" instruction in x86. PIL structure mandates that call produces a value that is assigned to a varaible. No such value exists in Intel architectures, so "call" returns "undefined".

Control Flow
------------

The PIL programs produced by the disassemblers are seqences of instructions. No jump or optional instructions are allowd inside a mnemonic. After each mnemonic an unlimited number of jumps is allowed. Each jump is associated with a guard which is a boolean PIL expression. If the guard is true, the jump is taken. A convetional "jmp" mnemonic in x86 can be modeled like this

.. code-block:: c++

  ...

Nevertheless PIL has a "call" instruction. This instruction has a single argument that specifis the address where a new function begins. No "return" instruction exists in PIL. Functions terminate after a sequence with no outgoing jumps is reached.

Generating Code
---------------

The textual representaion of PIL used previous examples can'b be used directly in the disassembler. The code is expected to generate the PIL structures itself. PIL is defined in the "value.hh" and "instr.hh" header files . These are part of the Panopticon library. A PIL instruction is an instance of the "instr" class. Its contructor needs the operation to use, its arguments and the variable or memory reference that receives the result of the operation:

.. code-block:: c++

  instr i(logic_xor{true,false},variable("a"));
  instr j(int_add{variable("b"),contant(55)},variable("c"));

Classes the represent PIL values are defined in "value.hh". These are either "constant", "variable", "memory" or "undefined". The "lvalue" type is a union of all value classes the can be the target of an assignment, "rvalue" combines all implemented value types.

The PIL operations are named <domain>_<operation> where <domain> is either "int" for operations accepting integer arguments, "logic" for operations on booleans or "univ" if both types are allowed. Keep in mind that "univ" operations do not allow mixing of types. All arguments need be either integers of booleans. Supported operations are:

- univ_phi
- ...

To make "instr" instance construction easier, the disassembler framework defines a "code_generator" class and give an instance of it to the semantic function of an opcode. The "code_generator" structure has methods for starting new mnemonics and appending PIL instructions to them.

.. code-block:: c++

  ---test a, b => a = a*55 + b
  st.mnemonic("test",2,{variable("a"),variable("b")},[&](void)
  {
  cg.add_i(variable("a"),cg.mul_i(variable("a"),constant(55)),variable("b"));
  cg.jump(st.address + 2);
  });

The code above add the 3 byte large mnemonic "test" to the current basic block. The mnemonic receives two arguments "a" and "b". When executed "test" computes "a * 55 + b", writes the value into "a" and jump the the next mnemonic. The code_generator methods come in two version. One is called with the arguments for the operations and returns a temporary variable with the result, another that accepts the target of the assignment as the first argument and the operands of the operation after that.

To make complex PIL expression more readable Panopticon includes overloads of most of the arithmetic and logic operators that behave like the code generator methods Thses overloads reside in the "po::dsel" namespace and are "activated" by including this namespace.

.. code-block:: c++

  using namespace po::dsel;

  st.mnemonic("test",2,{variable("a"),variable("b")},[&](void)
  {
  variable a("a"), variable b("b");
  cg.assign(a, a * 55 + b);
  cg.jump(st.address + 2);
  });

This code has the same semantic as the one above.
