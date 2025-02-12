use suffix::{ADJ, ADV, OBJ, VERB};

fn main() {
    // 賢い私は難しい問題を解ける
    let text = "sma-ta mio ba-ke estu sma-to";
    let ast = Node::parse(text);
    dbg!(ast.clone());
}

const SPACE: &str = " ";

/// 格変化
mod suffix {
    /// 名詞
    pub const OBJ: char = 'o';
    /// 動詞
    pub const VERB: char = 'u';
    /// 形容詞
    pub const ADJ: char = 'a';
    /// 副詞
    pub const ADV: char = 'e';
    /// 所有格
    pub const OWN: char = 'i';
}

#[derive(Clone, Debug)]
enum Node {
    Word {
        word: Noun,
        adj: Vec<Noun>,
    },
    Verb {
        verb: Noun,
        adv: Vec<Noun>,
        subj: Option<Box<Node>>,
        obj: Box<Node>,
    },
}

impl Node {
    fn parse(source: &str) -> Option<Self> {
        let tokens: Vec<&str> = source.split_whitespace().collect();
        if tokens.len() == 1 {
            if let Some(token) = tokens.first()?.strip_suffix(OBJ) {
                return Some(Node::Word {
                    adj: vec![],
                    word: Noun::parse(token)?,
                });
            }
        } else {
            let get_after = |x| Some(tokens.get(x..)?.join(SPACE));
            let get_ads = |x: char, srtidx: usize, endidx: usize| {
                let mut result = vec![];
                let mut index = 0;
                for i in tokens.get(srtidx..endidx)? {
                    if let Some(i) = i.strip_suffix(x) {
                        result.push(Noun::parse(i)?);
                        index += 1;
                    } else {
                        break;
                    }
                }
                Some((result, index))
            };

            {
                let mut index = 0;
                let mut advs = vec![];
                let mut obj = String::new();
                while index < tokens.len() {
                    let current = tokens.get(index)?;
                    if let Some(verb) = current.strip_suffix(VERB) {
                        return Some(Node::Verb {
                            verb: Noun::parse(verb)?,
                            adv: advs,
                            subj: if obj.is_empty() {
                                None
                            } else {
                                Some(Box::new(Node::parse(obj.trim())?))
                            },
                            obj: Box::new(Node::parse(&get_after(index + 1)?)?),
                        });
                    } else if let Some(adv) = current.strip_suffix(ADV) {
                        advs.push(Noun::parse(adv)?);
                    } else {
                        obj.push_str(&(current.to_string() + SPACE));
                    }
                    index += 1
                }
            }

            if tokens.first()?.ends_with(ADJ) {
                let (adjs, index) = get_ads(ADJ, 0, tokens.len())?;
                return Some(Node::Word {
                    word: Noun::parse(&get_after(index)?)?,
                    adj: adjs,
                });
            }
        }
        todo!()
    }
}

#[derive(Clone, Debug)]
enum Vocabulary {
    Tu,
    Mi,
    Est,
    Smart,
    Bak,
    Can,
    Fast,
    Hard,
    Problem,
}

impl Vocabulary {
    fn parse(source: &str) -> Option<Self> {
        Some(match source {
            "mi" => Self::Mi,
            "t:u" => Self::Tu,
            "est" => Self::Est,
            "sma-t" => Self::Smart,
            "ba-k" => Self::Bak,
            "can" => Self::Can,
            "fast" => Self::Fast,
            "ha-d" => Self::Hard,
            "problem" => Self::Problem,
            _ => return None,
        })
    }

    fn format(&self) -> String {
        match self {
            Self::Mi => "mi",
            Self::Tu => "t:u",
            Self::Est => "est",
            Self::Smart => "sma-t",
            Self::Bak => "ba-k",
            Self::Can => "can",
            Self::Fast => "fast",
            Self::Hard => "ha-d",
            Self::Problem => "problem",
        }
        .to_string()
    }
}

#[derive(Clone, Debug)]
struct Noun(Vec<Vocabulary>);

impl Noun {
    fn parse(source: &str) -> Option<Self> {
        let chars: Vec<char> = source.chars().collect();
        let mut index = 0;
        let mut position = 1;
        let mut result = vec![];
        while index < chars.len() {
            let noun = chars
                .get(index..position)?
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .concat();
            if let Some(noun) = Vocabulary::parse(&noun) {
                result.push(noun);
                index = position
            }
            position += 1
        }
        Some(Noun(result))
    }

    fn format(&self) -> String {
        self.0
            .iter()
            .map(|x| x.format())
            .collect::<Vec<String>>()
            .concat()
    }
}
