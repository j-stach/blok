
use crate::build::{ Props, Block, Layer };

trait LayerConnect<P: Props, B: Block<P>>: Layer<P, B> {

}


use super::Connectable;

impl<P: Props, B: Block<P>, L: Layer<P, B>> Connectable<P, B> for L {}

