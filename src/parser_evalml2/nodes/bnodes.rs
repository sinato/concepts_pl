use super::writer::RuleWriter;
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct BOpNode {
    pub i1: i32,
    pub i2: i32,
    pub op: String,
}
impl BOpNode {
    pub fn show<W: Write>(self, writer: &mut RuleWriter<W>) -> io::Result<()> {
        match self.op.as_ref() {
            "+" => writer.show_rule(
                None,
                self.i1.to_string() + " plus " + &self.i2.to_string(),
                (self.i1 + self.i2).to_string(),
                "B-Plus".to_string(),
                true,
                None,
                None,
                None,
            ),
            "*" => writer.show_rule(
                None,
                self.i1.to_string() + " times " + &self.i2.to_string(),
                (self.i1 * self.i2).to_string(),
                "B-Times".to_string(),
                true,
                None,
                None,
                None,
            ),
            _ => panic!("todo"),
        }
    }
}
