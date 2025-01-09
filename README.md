
# blok
Types and traits for graphlike 3D arrays.

## Usage
See [example 1](/example/1.rs) for a basic demonstration of building with blocks. <br>
See [example 2](/example/2.rs) for a demonstration of procedural node connection. <br>
See the [docs](https://docs.rs/blok/latest/blok/) for explanations of specific types and methods.

## Development
**Blok is a work-in-progress.** Current tasks can be tracked in [TODO.md](/TODO.md). <br>
#### In 0.0.2 
- Stack, Layer, Row are all generic types
- Added partial references
- Connections no longer rely on clone 
- Connection methods moved to a `Node` subtrait
- Alignments can be used on rows and layers during connection, as with blocks.
- Transformations temporarily removed
#### For 0.0.3 
- [ ] Improved transformations
- [ ] Methods for removing blocks 
- [ ] Connection and disconnection fleshed out
- [ ] Descriptive error types
- [ ] 1-based array indexing
- [ ] Tests and documentation improved
- [ ] TBD...

## Notes on style
1. Variable names in function declarations generally use the name of the type
(block, row, layer, etc.) when that type is expected as an argument, 
and will use the first letter of the type (b, r, l) 
when the argument expects a `usize` indicating the position 
of the corresponding element within a stack.
2. Methods that "find" a block index using layouts will fail with an error.
Methods that "get" references to blocks will treat that error as None,
representing the failure to find the block as an absence.
3. When building structures, blocks cannot be "inserted" at the last position of an element,
or in any other situation where the future location cannot be indexed as an existing block.
In such a scenario, you must use a method that "adds" it to the structure, instead.
Conventions like this exist for ease-of-development through the early stages
and will likely be corrected for improved ease-of-use as the crate is refined.
4. The build process is designed to be similar at each level of construction,
so if you are expecting to use a method that would correspond to another 
that exists for a different scale, but finding that it does not exist,
then it is probable that the method or function has not been implemented yet.
These corresponding methods will be fleshed out as the crate is refined, 
so that the build process is intuitively similar for each of the types.

