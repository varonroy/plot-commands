use super::DrawComand;

#[derive(Debug, Clone)]
pub enum Layout {
    VSplit(Vec<DrawComand>),
    HSplit(Vec<DrawComand>),
}
