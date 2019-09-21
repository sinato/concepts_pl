use super::super::nodes::get_depth_space;
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct BPlusNode {
    pub i1: i32,
    pub i2: i32,
}
impl BPlusNode {
    pub fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        write!(
            w,
            "{}{} plus {} is {} by B-Plus {{}}{}",
            get_depth_space(depth),
            self.i1,
            self.i2,
            self.i1 + self.i2,
            if with_newline { "\n" } else { "" }
        )
    }
}
