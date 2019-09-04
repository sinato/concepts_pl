use super::lexer::{Token, Tokens};

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Terms {
    terms: Vec<(String, i32)>,
}
impl Terms {
    pub fn new() -> Terms {
        let terms: Vec<(String, i32)> = Vec::new();
        Terms { terms }
    }
    pub fn push(&mut self, term: (String, i32)) {
        self.terms.push(term)
    }
    pub fn pop(&mut self) -> Option<(String, i32)> {
        self.terms.pop()
    }
    pub fn len(&self) -> usize {
        self.terms.len()
    }
    pub fn get_split_position(&self) -> (usize, String) {
        let mut priorities: HashMap<String, usize> = HashMap::new();
        priorities.insert("".to_string(), 0);
        priorities.insert("*".to_string(), 10);
        priorities.insert("-".to_string(), 20);
        priorities.insert("+".to_string(), 20);

        let mut split_position = 0;
        let mut priority: usize = 0;
        let mut ret_op: String = "".to_string();
        let terms = self.terms.clone();
        for (i, (operator, _)) in terms.into_iter().enumerate() {
            if priority
                <= *priorities
                    .get(&operator)
                    .expect("can not get operator priority")
            {
                split_position = i;
                priority = *priorities
                    .get(&operator)
                    .expect("can not get operator priority");
                ret_op = operator;
            }
        }
        (split_position, ret_op.to_string())
    }
    pub fn get_splitted_terms(&self, split_position: usize) -> (Terms, Terms) {
        let mut former: Vec<(String, i32)> = Vec::new();
        let mut latter: Vec<(String, i32)> = Vec::new();

        let terms = self.terms.clone();
        for (i, term) in terms.into_iter().enumerate() {
            if i < split_position {
                former.push(term);
            } else {
                latter.push(term);
            }
        }
        let former = Terms { terms: former };
        let mut latter = Terms { terms: latter };
        latter.rm_first_operator();
        (former, latter)
    }
    pub fn rm_first_operator(&mut self) {
        let mut new_terms: Vec<(String, i32)> = Vec::new();
        let terms = self.terms.clone().into_iter();
        for (i, (op, num)) in terms.enumerate() {
            if i == 0 {
                new_terms.push((String::from(""), num))
            } else {
                new_terms.push((op, num));
            }
        }
        self.terms = new_terms;
    }
}
