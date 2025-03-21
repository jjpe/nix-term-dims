# nix-term-dims

A utility to query the dimensions of the terminal it is run in.
Works on `Linux/x86-64` and `MacOS/AArch64`. Other Unixen may
or may not work, but are not supported and definitely not tested.

# Usage

Execute `nix-term-dims --columns --rows` to query both
the width (in columns) and the height (in rows).

It's also possible to only use one of those, to make querying
a single dimension easier without needing post-processing.
