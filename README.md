

# blok

Traits for graphlike 3D arrays. (Work in progress.) <br>
[Documentation](https://docs.rs/blok/latest/blok/)


## Usage
- Builder arguments follow add(where, what) schema ("Little endian?")
- Functional build process, returns error or self 
- Partial references return None when the index DNE
- Alignments create a connection "schedule" using indexes
- During connections, use the `'c` lifetime for references


## Future directions
I'm developing this crate to support another project I am working on, and decided
to split it off since it could be used more generally and may come in handy elsewhere.
<br>
Current tasks can be tracked in [TODO.md](/TODO.md). <br>


