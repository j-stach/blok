
# blok TODO:

## General code
- [ ] Cleaning
- [ ] Errors handled (thiserror)
- [ ] Optimization (in-place modification instead of cloning)
- [ ] Builder chain returning &self, &mut self, etc.
- [ ] 1-based indexing to make lengths vs. indexes easier?

----

## examples
- [ ] How to implement blok traits & connectors for custom types
- [ ] Build then stack,
- [ ] Build on stack (build stack in place)
- [ ] How to nest blok traits (ie, how to use a stack as a block)
- [ ] Implementing block for sync and send, connecting using clones
## tests
- [ ] block.rs 
- [ ] layer.rs
- [ ] stack.rs
- [ ] connect/mod.rs

----
- [ ] TODO! Needs `disconnect` method for Block trait
- [ ] Hide "connect", "clone", and other behavior behind feats

## types
- [ ] RowRef, LayerRef and StackRef for vector-matrix of in-place references (as opposed to clones)
- [ ] Connect gets clone version & reference version too
- [ ] Reduce overlap/repetition with traits? Or helper functions.

### row.rs 
- [ ] Integration with higher types
- [ ] Basic functions

### layer.rs 
- [ ] rotate_90/180/270

### stack.rs 
- [ ] set_layer 
- [ ] offset_xyz 
- [ ] collapse 
- [ ] fusion/merge_overlap 
- [ ] realize_volume


----

## docs
- Builder arguments follow add(where, what) schema ("Little endian?")
