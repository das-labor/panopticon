Regions and Layer
-----------------

Data in Panopticon is organized into regions. Each :cpp:class:`region` is a array of one byte
wide cells. On top of a region can be a number layer. A :cpp:class:`layer` spans part or all
of its region and transforms the content of cells inside.

Region
~~~~~~

Regions model continuous memory like RAM, flash memory or files. A region has a unique
name that is used to reference it and a size. The size is the number of cells in
a region. A cell either has a value between 0 and 255 or is undefined. Cells are
numbered in ascending order starting at 0.

Regions can be constructed from files or buffers in memory or be filled with
undefined values.

.. code-block:: c++

   // This region is named "file" and is filled with the contents of "path/to/file"
   region_loc file_region = region::wrap("file", blob("path/to/file"));

   // This region is named "buf" and is initialized with the contents of buf
   std::vector<uint8_t> buf = {1, 2, 3, 4, 5, 6, 7, 8, 9, 0};
   region_loc buf_region = region::wrap("buf", buf);

   // This region is named "undef" and is just 4k of undefined cells
   region_loc undefined_region = region::undefined("undef", 4096);


Reading from a region is done by calling :cpp:func:`read` on it. The function returns a
:cpp:class:`slab` instance that is a range of cells. Each cell is a :cpp:class:`tryte`
instance.

Layer
~~~~~

Layer transform parts of a region in some way. Instead of writing the contents
of a region directly the cells are covered with a layer. This allows changes to be
tracked. A region that models the RAM of a process could be covered with a layer
that replaces parts of this region with the contents of a file. This is an easy
way to model mapping files into process memory.

.. image:: layer.png

Layer can work as functions and take the contents of the region into
account. A layer could XOR each cell with a constant value to model the results
of unpacking a packed executable. Layer are also used to make regions writable.
Changing a cells value will add a layer on the region of the cell. This layer
will transform the written cell from its old value to the new one.

A layer instance has a name but no size. Instead, it is given a slab to
transform. Regions keep track with parts of its contents need to be transformed
using which layer. Layers are added to a region using :cpp:func:`add`.

.. code-block:: c++

   // Loading a Windows COM file

   // All accessable RAM is modeled as a single region
   region_loc reg = region::undefined("ram", 0xc0000000);

   // The file that's being loaded
   blob com("/path/to/file.com");

   // The layer that simulates mapping the COM file into RAM
   layer_loc mapping(new layer("file.com", com));

   // COM files are always mapped at 0100h
   reg.write().add(bound(0x100, 0x100 + com.size()), mapping);

Region Graph
~~~~~~~~~~~~

Regions can overlap each other. This allows Panopticon to display compressed
parts of a region. The decompression can't be modeled by a layer that transforms
the compressed data into its uncompressed state because the result is larger
than the source data. Layers are one-to-one transformations. The solution is to
create a new region wrapping the uncompressed data.

.. image:: zip-region.png

The new region covers the compressed data that is part of another region.
Panopticon models this relationship as an directed acyclic graph. The edges
from the covered region to the one covering it. Edges are annotated with the
area covered. The region graph has a root, a region with no incoming edges.
Every session has exactly one region graph.
