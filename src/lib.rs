
pub mod build;
pub use build::{
    Props, Block, Layer, Stack
};

pub mod connect;
pub use connect::*;

pub mod layout;
pub use layout::*;

/*
 Notes on use cases:
 volume & material-aware modeling & rendering
 connections for game physics
 matrix for game room, "objects" can be represented as collections
 of arranged properties that move between blocks
 */




