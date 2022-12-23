# markelf
A small program for embedding a string into the unused ELF header padding

Usage: `markelf <input file> <string>`

## Notice

**This is not guaranteed to work across all platforms.**

If a platform does not skip the padding bytes altogether, or if the ELF format changes to extend `e_ident`, there is a good chance this will not work.<br>
This is mainly just a short half-joke created out of boredom and sleep deprivation, so it is not intended for production code.
