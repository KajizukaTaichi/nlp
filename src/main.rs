use suffix::{ADJ, ADV, OBJ, OWN, VERB};

fn main() {
    // 賢い私は難しい問題を解ける
    let text = "sma-ti mio faste canu ha-da problemo";
    let ast = Node::parse(text);
    dbg!(ast.clone(), ast.map(|x| x.format()));
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
        own: Option<Box<Node>>,
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
        let source = source.trim();
        dbg!(&source);
        let tokens: Vec<&str> = source.split_whitespace().collect();
        if tokens.len() == 1 {
            if let Some(token) = tokens.first()?.strip_suffix(OBJ) {
                return Some(Node::Word {
                    word: Noun::parse(token)?,
                    adj: vec![],
                    own: None,
                });
            }
        } else {
            let get_after = |x| Some(tokens.get(x..)?.join(SPACE));
            let get_owns = |srtidx: usize| {
                let mut result = String::new();
                let mut index = 0;
                for i in tokens.get(srtidx..)? {
                    if let Some(i) = i.strip_suffix(OWN) {
                        result.push_str(&(i.to_string() + SPACE));
                        return Some((Node::parse(&(result.trim().to_string() + "o")), index + 1));
                    } else {
                        result.push_str(&(i.to_string() + SPACE));
                    };
                    index += 1;
                }
                None
            };

            if tokens.iter().any(|x| x.ends_with(VERB)) {
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

            if tokens.iter().any(|x| x.ends_with(OBJ)) {
                let (own, mut index) = get_owns(0).unwrap_or((None, 0));
                let mut adjs = vec![];
                while index < tokens.len() {
                    let current = tokens.get(index)?;
                    if let Some(obj) = current.strip_suffix(OBJ) {
                        return Some(Node::Word {
                            word: Noun::parse(obj)?,
                            own: if let Some(own) = own {
                                Some(Box::new(own))
                            } else {
                                None
                            },
                            adj: adjs,
                        });
                    } else if let Some(adj) = current.strip_suffix(ADJ) {
                        adjs.push(Noun::parse(adj)?)
                    }
                    index += 1
                }
            }
        }
        None
    }

    fn format(&self) -> String {
        match self {
            Node::Verb {
                verb,
                adv,
                subj,
                obj,
            } => {
                format!(
                    "{}{}{}u {}",
                    subj.clone()
                        .map(|x| x.format() + SPACE)
                        .unwrap_or("".to_string()),
                    {
                        let x = adv
                            .iter()
                            .map(|x| x.format())
                            .collect::<Vec<String>>()
                            .join(" ");
                        if x.is_empty() {
                            x
                        } else {
                            format!("{x}e ")
                        }
                    },
                    verb.format(),
                    obj.format()
                )
            }
            Node::Word { word, own, adj } => {
                format!(
                    "{}{}{}o",
                    own.clone()
                        .map(|x| {
                            let x = x.format();
                            if let Some(x) = x.strip_suffix(OBJ) {
                                x.to_string()
                            } else {
                                x
                            }
                        } + "i"
                            + SPACE)
                        .unwrap_or("".to_string()),
                    {
                        let x = adj
                            .iter()
                            .map(|x| x.format())
                            .collect::<Vec<String>>()
                            .join(" ");
                        if x.is_empty() {
                            x
                        } else {
                            format!("{x}a ")
                        }
                    },
                    word.format(),
                )
            }
        }
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
