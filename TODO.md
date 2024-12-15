
# blok 0.0.2 TODO:

- [x] Connection methods 
- [x] README
- [x] example 1
- [ ] debug tests and example: Indexing errors
- [x] clean up Cargo.toml
- [ ] clean up unused variables, yikes
- [ ] update TODO for next version
- [ ] publish update to crates.io

----

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
- [x] Basic use demonstration
- [ ] Transformations 
- [ ] Connections

## tests
- [ ] additional/variable blocks

----

## block
- [ ] `disconnect` method for Block trait to allow remove
- [ ] `remove`
- [ ] Hide "connect", "clone", and other behavior behind feats
- [ ] CreationInstruction factories

## types
- [x] Convert connect to reference version

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

