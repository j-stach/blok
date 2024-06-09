
use derive_more::{ Deref, DerefMut };
use serde::{ Serialize, Deserialize };


#[derive(Deref, DerefMut, Debug, Clone, Serialize, Deserialize)]
pub struct Layout(pub Vec<usize>);
impl Layout {
    pub fn new() -> Self { Self(Vec::new()) }

    pub fn wrap(vec: Vec<usize>) -> Self { Layout(vec) }

    // TBD "set_total" ? And have the total be a field, for easy access
    pub fn total(&self) -> usize {
        let mut total = 0usize;
        for e in self.iter() { total += e }
        total
    }

    pub fn row_start(&self, row: usize) -> Option<usize> {
        if *self.get(row)? == 0 { return None };
        let mut start = 0usize;
        for l in &self[0..row] {
            start += l
        }
        Some(start)
    }
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

#[macro_export] macro_rules! layout {
    () => { Layout::new() };
    ($($elem:expr),+ $(,)?) => {{ Layout(vec![$($elem),+]) }};
    ($elem:expr; $count:expr) => {{ Layout(vec![$elem; $count]) }};
}



