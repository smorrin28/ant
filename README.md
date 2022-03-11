### ANT

A simple assembly simulation written in rust. It features the following commands:

- DLOAD
- LOAD
- STORE
- ADD
- SUB
- MULT
- DIV
- JUMP
- JGT
- JGE
- JEQ
- JLE
- JLT
- END

You can run your programs by invoking the cli like this:

``ant path/to/file.asm``

In total, there are 16 registers which can only be used to store 8-bit unsigned integers.

Support for 32-bit signed integers (positive numbers) and comments is planned and will come soon.

Please not that this is an expirimental hobby project that I made to learn Rust so be sure to expect some bugs. 

In case you cannot run the packaged file on github, you can compile the project with ```cargo build``` and then run the compiled executable.