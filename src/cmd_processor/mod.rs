use crate::draw_command::DrawComand;

#[cfg(feature = "plotters")]
pub mod plotters;

pub trait CmdProcessor {
    fn proces(&self, cmd: &DrawComand);
}
