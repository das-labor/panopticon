Usage
=====

The current version only supports AVR and has no ELF or PE loader yet.
To test Panopticon you need relocated AVR code. Such a file is prepared in
``lib/test/sosse``.

.. code-block:: bash

  qt/qtpanopticon -a ../panopticon/lib/test/sosse

Or, you can start Panopticon without command line parameters and
select the test file manually by starting a new session.
