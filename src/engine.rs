use crate::*;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
}

impl Value {
    fn as_number(&self) -> Option<f64> {
        match self {
            Self::Number(x) => Some(*x),
            _ => None,
        }
    }

    fn as_string(&self) -> Option<String> {
        match self {
            Self::String(x) => Some(x.clone()),
            _ => None,
        }
    }
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
                let lhs = self.eval(&*subj)?;
                let rhs = self.eval(&*obj)?;

                if verb.first()?.0 == "nam" {
                    Some(match verb.last()?.0.as_str() {
                        "a*d" => Value::Number(lhs.as_number()? + rhs.as_number()?),
                        "pul" => Value::Number(lhs.as_number()? - rhs.as_number()?),
                        "kak" => Value::Number(lhs.as_number()? * rhs.as_number()?),
                        "div" => Value::Number(lhs.as_number()? / rhs.as_number()?),
                        _ => return None,
                    })
                } else if verb.first()?.0 == "car" && verb.get(1)?.0 == "a-l" {
                    Some(match verb.last()?.0.as_str() {
                        "a*d" => Value::String(lhs.as_string()? + &rhs.as_string()?),
                        _ => return None,
                    })
                } else if verb.first()?.0 == "est" {
                    self.scope.insert(lhs.as_string()?, rhs.clone());
                    Some(rhs)
                } else {
                    None
                }
            }
            Node::Verb {
                verb: Noun(verb),
                adv,
                subj: None,
                obj,
            } if adv.is_empty() => {
                let rhs = self.eval(&*obj)?;
                if verb.first()?.0 == "ge*t" {
                    self.scope.get(&rhs.as_string()?).cloned()
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
                .unwrap_or(Some(Value::String(word.format()))),
            _ => None,
        }
    }
}
