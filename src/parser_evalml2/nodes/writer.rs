use super::super::environment::Environment;
use super::super::nodes::{get_depth_space, RuleNode};
use super::bnodes::BOpNode;

use std::io::{self, Write};

pub struct RuleWriter<W> {
    w: W,
    depth: usize,
}
impl<W: Write> RuleWriter<W> {
    pub fn new(w: W, depth: usize) -> RuleWriter<W> {
        RuleWriter { w, depth }
    }

    pub fn write_nl(&mut self) {
        let _ = write!(self.w, "\n");
    }

    fn inc_depth(&mut self) {
        self.depth += 2;
    }
    fn dec_depth(&mut self) {
        self.depth -= 2;
    }

    pub fn show_rule(
        &mut self,
        environment: Option<Environment>,
        expression_str: String,
        evalto_str: String,
        rule_str: String,
        is_bnode: bool,
        premise1: Option<RuleNode>,
        premise2: Option<RuleNode>,
        premise3: Option<BOpNode>,
    ) -> io::Result<()> {
        let environment_str = match environment.clone() {
            Some(env) => env.to_string(),
            None => "".to_string(),
        };
        let eq_str = if is_bnode { "is" } else { "evalto" };
        let _ = write!(
            self.w,
            "{}{}{} {} {} by {} {{",
            get_depth_space(self.depth),
            environment_str,
            expression_str,
            eq_str.to_string(),
            evalto_str,
            rule_str,
        );

        let mut eol_necessity = false;

        self.inc_depth();
        if let Some(premise) = premise1 {
            let _ = write!(self.w, "\n");
            let _ = premise.show(self);
            eol_necessity = true;
        }
        if let Some(premise) = premise2 {
            let _ = write!(self.w, ";\n");
            let _ = premise.show(self);
            eol_necessity = true;
        }
        if let Some(premise) = premise3 {
            let _ = write!(self.w, ";\n");
            let _ = premise.show(self);
            eol_necessity = true;
        }
        self.dec_depth();
        if eol_necessity {
            write!(self.w, "\n{}}}", get_depth_space(self.depth))
        } else {
            write!(self.w, "}}")
        }
    }
}
