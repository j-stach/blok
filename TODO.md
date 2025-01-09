
# blok 0.0.2 TODO:

- [x] Connection methods 
- [x] README
- [x] example 1
- [x] clean up Cargo.toml
- [x] clean up unused variables, yikes
- [x] refactor modules 
- [x] Helper functions and types for ranges etc.
- [x] NodeBlock subtrait for connections
- [x] Tests and examples refactored, doc tests in comments
- [ ] Tests and examples debugged
- [x] previously_available_recursion_helper functions for Layer
- [x] update TODO for next version
- [ ] publish update to crates.io

----
# 0.0.3 TODO:

## General code
- [x] Cleaning & refactor old code
- [ ] Errors handled (thiserror)
- [x] Optimization (in-place modification instead of cloning)
- [x] Standardize and DRY, more utility functions
- [ ] Optimization (not too dry that I'm repeating checks)
- [ ] finish missing methods
- [x] Builder chain returning &self, &mut self, etc.
- [ ] 1-based indexing to make lengths vs. indexes easier? 

----

## examples
- [x] Basic use demonstration
- [ ] Transformations 
- [ ] Connections

## tests
- [ ] additional/variable blocks

----

## block
- [ ] `disconnect` method for Block trait to allow `remove`
- [ ] Hide "connect", "clone", and other behavior behind feats
- [ ] CreationInstruction factories

### row 
- [x] Integration with higher types
- [x] Basic functions

### layer 
- [x] clone_into_rows/set_from
- [x] pad x, y, row
- [ ] tumble_90/180/270

### stack 
- [x] set_layer 
- [ ] offset_xyz 
- [ ] collapse 
- [x] realize_volume
- [ ] Vertical slices for partial ref
- [x] insert_row

----

TODO:
- Change partial refs to return empty vec on None, and None on error.
