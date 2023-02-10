use crate::{
    grid::{config::GridConfig, dimension::Dimension},
    records::into_records::truncate_records::Width,
    records::Records,
};

#[derive(Debug)]
pub struct IterTableDimension<'a> {
    width: Width<'a>,
    height: usize,
}

impl<'a> IterTableDimension<'a> {
    pub fn new(width: Width<'a>, height: usize) -> Self {
        Self { width, height }
    }
}

impl Dimension for IterTableDimension<'_> {
    fn estimate<R: Records>(&mut self, _: R, _: &GridConfig) {}

    fn get_width(&self, column: usize) -> usize {
        self.width.get(column)
    }

    fn get_height(&self, _: usize) -> usize {
        self.height
    }
}
