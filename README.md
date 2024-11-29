

# blok

Traits for graphlike 3D arrays. (Work in progress.) <br>
[Documentation](https://docs.rs/blok/latest/blok/)


## Usage
### Building a stack
- Builder arguments follow add(where, what) schema ("Little endian?")
- Functional build process, returns error or self 
- Layouts and blocks 
### Partial references
- Partial references return None when the index or range DNE
### Transformations
- Transformations reorganize blocks by cloning, 
modifying, and resetting the stack in place
### Connecting blocks
- Alignments create a connection "schedule" using indexes, 
for layer row and block
- During connections, use the `'c` lifetime for references


## Future directions
I'm developing this crate to support another project I am working on, and decided
to split it off since it could be used more generally and may come in handy elsewhere.
<br>
Current tasks can be tracked in [TODO.md](/TODO.md). <br>


