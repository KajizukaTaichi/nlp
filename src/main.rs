use suffix::{ADJ, OBJ, VERB};

fn main() {
    println!("Hello, world!");
    // 賢い私は難しい問題を解ける
    let text = "smartest";
    let ast = Noun::parse(text);
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
            let get_token = |x| Some(tokens.get(x..)?.join(SPACE));
            let get_ads = |x: char, srtidx: usize| {
                let mut result = vec![];
                let mut index = 0;
                for i in tokens.get(srtidx..)? {
                    if i.ends_with(x) {
                        result.push(Noun::parse(i)?);
                        index += 1;
                    } else {
                        break;
                    }
                }
                Some((result, index))
            };
            if tokens.first()?.ends_with(ADJ) {
                let (adjs, index) = get_ads(ADJ, 0)?;
                return Some(Node::Word {
                    word: Noun::parse(&get_token(index)?)?,
                    adj: adjs,
                });
            } else if tokens.first()?.ends_with(VERB) {
                let (adjs, index) = get_ads(ADJ, 0)?;
                return Some(Node::Word {
                    word: Noun::parse(&get_token(index)?)?,
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
            "smart" => Self::Smart,
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
            Self::Smart => "smart",
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
