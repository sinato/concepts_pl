mod lexer;

use lexer::{Lexer, Token, Tokens};

#[derive(Debug, PartialEq, Clone)]
enum RuleNode {
    PZero(PZeroNode),
}
impl RuleNode {
    fn new(tokens: &mut Tokens) -> RuleNode {
        let rule = PZeroNode::new(tokens);
        RuleNode::PZero(rule)
    }

    fn show(self) {
        match self {
            RuleNode::PZero(node) => node.show(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct PZeroNode {
    nat_num: usize,
}
impl PZeroNode {
    fn new(tokens: &mut Tokens) -> PZeroNode {
        tokens.pop(); // consume Z
        tokens.pop(); // consume plus
        let nat_num: usize = if let Some(Token::Zero(_)) = tokens.pop() {
            0
        } else {
            panic!("")
        }; // consume num
        tokens.pop(); // consume is
        tokens.pop(); // consume Z
        PZeroNode { nat_num }
    }

    fn show(self) {
        println!("Z plus {} is Z by P-Zero {{}}", "Z");
    }
}

fn main() {
    let judgement = "Z plus Z is Z".to_string();
    println!("judgement ========================");
    println!("{}", judgement);

    let lexer = Lexer::new();
    let mut tokens = lexer.lex(judgement);
    // dbg!(&tokens);
    let node = RuleNode::new(&mut tokens);
    // dbg!(&node);
    //
    println!("derivation tree ==================");
    node.show();
    println!("==================================");
}
