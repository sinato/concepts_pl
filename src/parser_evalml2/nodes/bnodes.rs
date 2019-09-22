use super::super::nodes::get_depth_space;
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct BOpNode {
    pub i1: i32,
    pub i2: i32,
    pub op: String,
}
impl BOpNode {
    pub fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        match self.op.as_ref() {
            "+" => write!(
                w,
                "{}{} plus {} is {} by B-Plus {{}}{}",
                get_depth_space(depth),
                self.i1,
                self.i2,
                self.i1 + self.i2,
                if with_newline { "\n" } else { "" }
            ),
            "*" => write!(
                w,
                "{}{} times {} is {} by B-Times {{}}{}",
                get_depth_space(depth),
                self.i1,
                self.i2,
                self.i1 * self.i2,
                if with_newline { "\n" } else { "" }
            ),
            _ => panic!("todo"),
        }
    }
}
