
# blok TODO:

## General code
- [ ] Cleaning & refactor old code
- [ ] Errors handled (thiserror)
- [ ] Optimization (in-place modification instead of cloning)
- [ ] Standardize and DRY, more utility functions
- [ ] Optimization (not too dry that I'm repeating checks)
- [x] Builder chain returning &self, &mut self, etc.
- [ ] 1-based indexing to make lengths vs. indexes easier?

----

## examples
- [ ] Basic use demonstration

## tests
### block
- [ ] additional/variable blocks
### stack

----
- [ ] TODO! Needs `disconnect` method for Block trait
- [ ] Hide "connect", "clone", and other behavior behind feats

## types
- [ ] Connect gets clone version & reference version too

### row.rs 
- [ ] Integration with higher types
- [ ] Basic functions

### layer.rs 
- [ ] clone_into_rows/set_from
- [ ] pad x, y, row
- [ ] rotate_90/180/270

### stack.rs 
- [ ] set_layer 
- [ ] offset_xyz 
- [ ] collapse 
- [ ] fusion/merge_overlap 
- [ ] realize_volume
- [ ] Vertical slices for partial ref
- [ ] insert_row

## order 
### layout
- [ ] empty_row
- [ ] initialize

----

## docs
- Builder arguments follow add(where, what) schema ("Little endian?")
- Functional build process, returns error or self 
- Partial references return None when the index DNE
