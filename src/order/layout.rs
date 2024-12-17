
use derive_more::{ Deref, DerefMut };
use serde::{ Serialize, Deserialize };


/// Represents the "shape" of the array layer for easy indexing.
#[derive(Deref, DerefMut, Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Layout(pub(crate) Vec<usize>);

impl Layout {

    /// Create a default (empty) layout.
    pub fn new() -> Self { 
        Self(Vec::new()) 
    }

    /// Create a Layout from a Vec<usize>.
    pub fn wrap(vec: Vec<usize>) -> Self { 
        Layout(vec) 
    }

    /// Count the number of blocks represented by the layout.
    pub fn total(&self) -> usize {
        let mut total = 0usize;
        for e in self.iter() { total += e }
        total
    }

    /// Check if the layout contains enough rows to index.
    /// Returns an error if the row index isn't found within the layout.
    pub fn row_exists(&self, r: usize) -> anyhow::Result<()> {
        if self.len() > r { Ok(()) } 
        else {
            Err(anyhow::anyhow!("Row index exceeds number of rows in layout"))
        }
    }

    /// Check of the chosen row is empty.
    /// Returns an error if the row index isn't found within the layout.
    pub fn row_is_empty(&self, r: usize) -> anyhow::Result<bool> {
        self.row_exists(r)?;
        let row_len = self.get(r).expect("Row exists");
        Ok(*row_len == 0) 
    }

    /// Find the block index for the start of a row.
    /// Returns an error if the row index isn't found within the layout.
    /// Returns None if the row is empty (contains no blocks).
    pub fn row_start(&self, r: usize) -> anyhow::Result<Option<usize>> {
        
        // If the row is empty, there will be no start (None).
        if self.row_is_empty(r)? { return Ok(None) }

        let mut start = 0usize;
        for l in &self[0..r] {
            start += l
        }

        Ok(Some(start))
    }

    /// Find the block index for the end of a row.
    /// Returns an error if the row index isn't found within the layout.
    /// Returns None if the row is empty (contains no blocks).
    pub fn row_end(&self, r: usize) -> anyhow::Result<Option<usize>> {
        
        // If the row is empty, there will be no start (None).
        if let Some(start) = self.row_start(r)? {
            let row_len = self.get(r).expect("Row exists");
            Ok(Some(start + row_len - 1))
        } else { 
            Ok(None)
        }
    }

    /// Get a range representing the layout row from start to end.
    /// Returns an error if the row index isn't found within the layout.
    /// Returns None if the row is empty (contains no blocks).
    pub fn row_range(&self, r: usize) -> anyhow::Result<Option<(usize, usize)>> {

        // If the row is empty, there will be no start (None).
        if let Some(start) = self.row_start(r)? {
            let end = self.row_end(r)
                .expect("Row exists")
                .expect("Row is not empty");
            Ok(Some((start, end)))
        } else {
            Ok(None)
        } 
    }

}

impl FromIterator<usize> for Layout {
    fn from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Self {
        Layout(iter.into_iter().collect())
    }
}

/// Macro for easy layout creation. Works like `vec![]`.
/// If the compiler gives you trouble, try changing your brackets to parentheses:
/// ```
///let layouts = vec![layout!(), layout!(1), layout!(2, 2), layout!(3; 3)];
/// ```
#[macro_export] macro_rules! layout {
    () => { Layout::new() };
    ($($elem:expr),+ $(,)?) => {{ Layout::wrap(vec![$($elem),+]) }};
    ($elem:expr; $count:expr) => {{ Layout::wrap(vec![$elem; $count]) }};
}


