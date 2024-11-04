
use super::*;

impl<B: Block> Stack<B> {

    /// Clone a layer from the stack and return it as a new entity.
    pub fn clone_layer(&self, l: usize) -> Option<Layer<B>> {
        let layouts = self.layouts();
        if l > layouts.len() { 
            return None 
        }

        let layout = &layouts[l];
        let start = self.find_layer_start(l)?;
        let end = start + layout.total();

        let mut layer = Layer::new();
        layer.set_from_layout(
            layout.clone(), 
            self.blocks()[start..end].to_vec()
        )
        .expect("Layout corrupted"); // TODO Error

        Some(layer)
    }

    /// Find the block index for the start of the layer.
    pub fn find_layer_start(&self, l: usize) -> Option<usize> {
        let layouts = self.layouts();
        if l > layouts.len() { 
            return None 
        }

        let mut start = 0usize;
        for layout in &layouts[0..l] {
            start += layout.total()
        }

        Some(start)
    }

    /// Get a reference to the block at the given index.
    pub fn get_block<'stack>(
        &'stack self,
        layer: usize,
        row: usize,
        mut index: usize
    ) -> Option<&'stack B> {

        let layout = &self.layouts()[layer];
        let l_start = self.find_layer_start(layer)?;
        if row > layout.len() { 
            return None 
        }

        let r_start = {
            let mut sum = 0usize;
            layout[0..row].iter().for_each(|r| sum += r);
            sum
        };

        if index > layout[row] { return None }
        index += l_start + r_start;

        Some(&self.blocks()[index])
    }
}

