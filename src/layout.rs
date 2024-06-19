
use derive_more::{ Deref, DerefMut };
use serde::{ Serialize, Deserialize };


/// Represents the "shape" of the array for easy indexing.
#[derive(Deref, DerefMut, Debug, Clone, Serialize, Deserialize)]
pub struct Layout(pub Vec<usize>);

impl Layout {
    /// Create a default (empty) layout.
    pub fn new() -> Self { Self(Vec::new()) }

    /// Create a Layout from a Vec<usize>.
    pub fn wrap(vec: Vec<usize>) -> Self { Layout(vec) }

    // TBD "set_total" ? And have the total be a field, for easy access
    /// Count the number of blocks represented by the layout.
    pub fn total(&self) -> usize {
        let mut total = 0usize;
        for e in self.iter() { total += e }
        total
    }

    /// Find the block index for the start of a row.
    pub fn row_start(&self, row: usize) -> Option<usize> {
        if *self.get(row)? == 0 { return None };
        let mut start = 0usize;
        for l in &self[0..row] {
            start += l
        }
        Some(start)
    }

    /// Find the block index for the end of a row.
    pub fn row_end(&self, row: usize) -> Option<usize> {
        if *self.get(row)? == 0 { return None };
        let mut end = 0usize;
        for l in &self[0..=row] {
            end += l - 1
        }
        Some(end)
    }
}

impl FromIterator<usize> for Layout {
    fn from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Self {
        Layout(iter.into_iter().collect())
    }
}

/// Macro for easy layout creation. Works like vec![].
#[macro_export] macro_rules! layout {
    () => { Layout::new() };
    ($($elem:expr),+ $(,)?) => {{ Layout(vec![$($elem),+]) }};
    ($elem:expr; $count:expr) => {{ Layout(vec![$elem; $count]) }};
}



