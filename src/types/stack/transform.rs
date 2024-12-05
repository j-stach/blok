
use super::*;

//
// TODO: 
// Transformations without cloning?
//

/// Transformations
impl<B: Block> Stack<B> {

    /// Split each layer into two at the given row number. Leaves the original in place.
    pub fn split_x(&mut self, split: usize) -> Self {
        let mut old = self.clone_into_layers();
        let mut new = Vec::new();

        for layer in old.iter_mut() {
            new.push(layer.split_x(split))
        }

        self.set_from_layers(old);

        let mut stack = Self::new();
        stack.set_from_layers(new);
        stack
    }

    /// Split each layer into two by splitting each row at index given.
    pub fn split_y(&mut self, split: usize) -> Self {
        let mut old = self.clone_into_layers();
        let mut new = Vec::new();

        for layer in old.iter_mut() {
            new.push(layer.split_y(split))
        }

        self.set_from_layers(old);

        let mut stack = Self::new();
        stack.set_from_layers(new);
        stack
    }

    /// Split the stack in two at the given layer.
    pub fn split_z(&mut self, split: usize) -> Self {
        let mut old = self.clone_into_layers();
        let new = old.split_off(split);

        self.set_from_layers(old);

        let mut stack = Self::new();
        stack.set_from_layers(new);
        stack
    }

    /// Flip each layer across the Y axis, reversing the sequence of rows.
    pub fn flip_x(&mut self) {
        let mut flipped = self.clone_into_layers();
        flipped.iter_mut()
            .for_each(|s| s.flip_x() );

        self.set_from_layers(flipped)
    }

    /// Flip each layer across the X axis, reversing the order of blocks within the rows.
    pub fn flip_y(&mut self) {
        let mut flipped = self.clone_into_layers();
        flipped.iter_mut()
            .for_each(|s| s.flip_y() );

        self.set_from_layers(flipped)
    }

    /// Reverses the order of stack layers.
    pub fn flip_z(&mut self) {
        let flipped = self.clone_into_layers()
            .into_iter()
            .rev()
            .collect();

        self.set_from_layers(flipped)
    }

    /// Stitch each layer to the corresponding layer in the other stack.
    pub fn stitch_x(&mut self, other: &mut Self) {
        let mut these = self.clone_into_layers();
        let those = other.clone_into_layers();

        these.iter_mut()
            .zip(those.into_iter())
            .for_each(|(s, mut o)| s.stitch_x(&mut o) );

        self.set_from_layers(these)
    }

    /// Stitch rows from each layer to the corresponding layer in the other stack.
    pub fn stitch_y(&mut self, other: &mut Self) {
        let mut these = self.clone_into_layers();
        let those = other.clone_into_layers();

        these.iter_mut()
            .zip(those.into_iter())
            .for_each(|(s, mut o)| s.stitch_y(&mut o) );

        self.set_from_layers(these)
    }

    /// Stack the entire other stack atop this stack's layers.
    pub fn stitch_z(&mut self, other: &mut Self) {
        self.layouts_mut().append(other.layouts_mut());
        self.blocks_mut().append(other.blocks_mut());
    }

    /// Stitch an x-flipped clone (after this stack's existing rows).
    pub fn mirror_x(&mut self) {
        let mut mirror = self.clone();
        mirror.flip_x();
        self.stitch_x(&mut mirror)
    }

    /// Stitch an y-flipped clone (after this stack's existing rows).
    pub fn mirror_y(&mut self) {
        let mut mirror = self.clone();
        mirror.flip_y();
        self.stitch_y(&mut mirror)
    }

    /// Stitch an z-flipped clone (atop this stack's existing rows).
    pub fn mirror_z(&mut self) {
        let mut mirror = self.clone();
        mirror.flip_z();
        self.stitch_z(&mut mirror)
    }

    /// Merge the corresponding layers of two stacks by alternating rows.
    /// The resulting layers will begin with a row originating from "self".
    pub fn riffle_x(&mut self, other: &mut Self) {
        let mut these = self.clone_into_layers();
        let those = other.clone_into_layers();

        these.iter_mut()
            .zip(those.into_iter())
            .for_each(|(s, mut o)| s.riffle_x(&mut o) );

        self.set_from_layers(these)
    }

    /// Merge the corresponding layers of two stacks by alternating indices for corresponding rows.
    /// The resulting layers' rows will begin with blocks originating from "self".
    pub fn riffle_y(&mut self, other: &mut Self) {
        let mut these = self.clone_into_layers();
        let those = other.clone_into_layers();

        these.iter_mut()
            .zip(those.into_iter())
            .for_each(|(s, mut o)| s.riffle_y(&mut o) );

        self.set_from_layers(these)
    }

    /// Merge two stacks by alternating layers.
    /// The new stack will begin with a layer from "self".
    pub fn riffle_z(&mut self, other: &mut Self) {
        let these = self.clone_into_layers();
        let those = other.clone_into_layers();

        let riffled: Vec<Layer<B>> = these.into_iter()
            .zip(those.into_iter())
            .flat_map(|(r, o)| vec![r, o])
            .collect();

        self.set_from_layers(riffled)
    }

}

