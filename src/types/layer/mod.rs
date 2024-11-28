
pub mod build;
pub mod void;
pub mod partial;
pub mod clone;
//pub mod connect;
//pub mod transform;

use crate::{ Block, Layout };

/// Holds a grid of blocks in a single vector, 
/// with the layout tracked separately.
#[derive(Debug, Default, Clone)]
pub struct Layer<B: Block> {
    /// Represents the organization of blocks in the array.
    layout: Layout,
    /// Blok assumes you want to store the block array contiguously in memory.
    blocks: Vec<B>
}

/// Field access methods:
impl<B: Block> Layer<B> {

    /// Create an empty layer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a reference to the blocks of the layer.
    pub fn blocks(&self) -> &Vec<B> { 
        &self.blocks 
    }

    /// Get a mutable reference to the blocks of the layer.
    pub(crate) fn blocks_mut(&mut self) -> &mut Vec<B> { 
        &mut self.blocks 
    }

    /// Get a reference to the layout of the layer.
    pub fn layout(&self) -> &Layout { 
        &self.layout 
    }

    /// Get a mutable reference to the layout of the layer.
    pub(crate) fn layout_mut(&mut self) -> &mut Layout { 
        &mut self.layout 
    }

    /// Set layer data arbitrarily.
    pub fn set_from_layout(
        &mut self, 
        layout: Layout, 
        blocks: Vec<B>
    ) -> anyhow::Result<()> { // TODO Error
        
        if layout.total() != blocks.len() { 
            return Err(anyhow::anyhow!("Size mismatch"))
            // TODO: Error
        }

        *self.layout_mut() = layout;
        *self.blocks_mut() = blocks;

        Ok(())
    }
    
}

