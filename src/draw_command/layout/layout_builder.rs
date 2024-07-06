use super::Layout;
use crate::draw_command::{DrawComand, IntoDrawCommand};

pub trait DrawComandGenerator {
    fn generate(&mut self);
}

pub struct LayoutBuilder(Layout);

impl std::default::Default for LayoutBuilder {
    fn default() -> Self {
        Self(Layout::Box(Box::new(DrawComand::Blank)))
    }
}

impl LayoutBuilder {
    pub fn r#box(self, cmd: impl IntoDrawCommand) -> Self {
        LayoutBuilder(Layout::Box(Box::new(cmd.into_draw_command())))
    }

    pub fn vsplit(self, cmds: impl IntoIterator<Item = DrawComand>) -> Self {
        LayoutBuilder(Layout::VSplit(cmds.into_iter().collect()))
    }

    pub fn hsplit(self, cmds: impl IntoIterator<Item = DrawComand>) -> Self {
        LayoutBuilder(Layout::HSplit(cmds.into_iter().collect()))
    }

    pub fn grid_with_rows(self, cmds: Vec<DrawComand>, rows: usize) -> Self {
        LayoutBuilder(Layout::Grid {
            commands: cmds,
            constraint: super::GridConstraint::Rows(rows),
        })
    }

    pub fn grid_with_cols(self, cmds: Vec<DrawComand>, cols: usize) -> Self {
        LayoutBuilder(Layout::Grid {
            commands: cmds,
            constraint: super::GridConstraint::Columns(cols),
        })
    }

    pub fn build(self) -> Layout {
        self.0
    }
}
