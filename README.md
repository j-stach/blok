
# blok (WIP)
Traits for graphlike 3D arrays.

## Usage
See [example 1](/example/1.rs) for a basic demonstration of the library. <br>
See the [docs](https://docs.rs/blok/latest/blok/) 
for explanations of specific types and methods.

## Development
Blok is a work-in-progress. Current tasks can be tracked in [TODO.md](/TODO.md). <br>
#### In 0.0.2 
- Stack, Layer, Row are all generic types
- Added partial references
- Connections no longer rely on clone 
- Alignments can be used on rows and layers during connection 
#### For 0.0.3 
- [ ] Error types
- [ ] 1-based array indexing
- [ ] Tests improved
- [ ] TBD...

## Notes on style
1. Variable names in function declarations generally use the name of the type
(block, row, layer, etc.) when that type is expected as an argument, 
and will use the first letter of the type (b, l, r, ...) 
when the argument expects a `usize` indicating the position 
of the corresponding element within a stack.
2. TODO: Explain use of Err and None when finding blocks. 
If it tries to find a block index using layouts, it will fail with an error.
Methods that get references to blocks will treat that error as None,
representing the failure to find the block.
Advice is to use the high-level Option methods.
Methods that reference empty rows return empty vectors.

