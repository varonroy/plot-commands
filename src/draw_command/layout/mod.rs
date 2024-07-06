#[cfg(feature = "builder")]
pub mod layout_builder;

use super::DrawComand;

#[derive(Debug, Clone, Copy)]
pub enum GridConstraint {
    Rows(usize),
    Columns(usize),
}

impl GridConstraint {
    pub fn calculate_rows_cols(&self, num_elements: usize) -> (usize, usize) {
        match *self {
            Self::Rows(rows) => (rows, num_elements.div_ceil(rows)),
            Self::Columns(cols) => (num_elements.div_ceil(cols), cols),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Layout {
    Box(Box<DrawComand>),
    VSplit(Vec<DrawComand>),
    HSplit(Vec<DrawComand>),
    Grid {
        commands: Vec<DrawComand>,
        constraint: GridConstraint,
    },
}
