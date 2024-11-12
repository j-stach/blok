
use super::*;

/// Functions for cloning the layer to and from different divisions.
impl<B: Block> Layer<B> {

    // TODO:
    // clone_into_rows
    // set_from_rows

    /// Clone the layer into a matrix of blocks.
    pub fn clone_into_blocks(&self) -> Vec<Vec<B>> {
        // Cannot clone into blocks if Layout is corrupted.
        assert_eq!(self.layout().total(), self.blocks().len());
        
        let mut clone = self.blocks().clone();
        let mut blocks = Vec::new();

        for r in self.layout().iter() {
            let (row, rest) = clone.split_at(*r);
            blocks.push(row.to_vec());
            clone = rest.to_vec();
        }

        blocks
    }

    /// Overwrite a layer's values from a matrix of blocks.
    pub fn set_from_blocks(&mut self, blocks: Vec<Vec<B>>) {
        self.layout = blocks.iter()
            .map(|v| v.len())
            .collect();

        self.blocks = blocks.into_iter()
            .flatten()
            .collect();
    }

}
