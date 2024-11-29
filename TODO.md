
# blok TODO:

## General code
- [x] Cleaning & refactor old code
- [ ] Errors handled (thiserror)
- [x] Optimization (in-place modification instead of cloning)
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
- [x] Convert connect to reference version

### row.rs 
- [x] Integration with higher types
- [x] Basic functions

### layer.rs 
- [x] clone_into_rows/set_from
- [x] pad x, y, row
- [ ] rotate_90/180/270

### stack.rs 
- [x] set_layer 
- [ ] offset_xyz 
- [ ] collapse 
- [x] realize_volume
- [ ] Vertical slices for partial ref
- [x] insert_row

----

