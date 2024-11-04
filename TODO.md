
# blok TODO:

## General code
- [ ] Cleaning
- [ ] Errors handled (thiserror)
- [ ] Optimization (in-place modification instead of cloning)

----

## examples
- [ ] How to implement blok traits & connectors for custom types
- [ ] How to nest blok traits (ie, how to use a stack as a block)
## tests
- [ ] block.rs 
- [ ] layer.rs
- [ ] stack.rs
- [ ] connect/mod.rs

----

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


