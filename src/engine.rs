use crate::*;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Symbol(String),
}

#[derive(Debug, Clone)]
pub struct Engine {
    pub scope: HashMap<String, Value>,
}

impl Engine {
    pub fn eval(&mut self, ast: &Node) -> Option<Value> {
        match ast.clone() {
            Node::Verb {
                verb: Noun(verb),
                adv,
                subj: Some(subj),
                obj,
            } if adv.is_empty() => {
                if let (Some(lhs), Some(rhs)) = (self.eval(&*subj), self.eval(&*obj)) {
                    if let (Value::Number(lhs), Value::Number(rhs)) = (lhs.clone(), rhs.clone()) {
                        Some(match verb.first()?.0.as_str() {
                            "a*d" => Value::Number(lhs + rhs),
                            "pul" => Value::Number(lhs - rhs),
                            "kak" => Value::Number(lhs * rhs),
                            "div" => Value::Number(lhs / rhs),
                            _ => return None,
                        })
                    } else if let Value::Symbol(lhs) = lhs {
                        Some(match verb.first()?.0.as_str() {
                            "est" => {
                                self.scope.insert(lhs, rhs.clone());
                                rhs
                            }
                            _ => return None,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Node::Word {
                word,
                own: None,
                adj: _,
            } => word
                .format()
                .parse()
                .map(|x| Some(Value::Number(x)))
                .unwrap_or(Some(
                    self.scope
                        .get(&word.format())
                        .cloned()
                        .unwrap_or(Value::Symbol(word.format())),
                )),
            _ => None,
        }
    }
}
