use suffix::{ADJ, ADV, OBJ, OWN, VERB};

fn main() {
    println!("# Komona Lange-zi prosactist\n");
    for text in [
        "c^u yuo estu i-tcana homa-lo",
        "d*ii komp^u-tekta programengo prosactu menya de-to",
        "mio stronge k^omavu internacia-la anarkiizmi movesto inu bes^mondo",
        "d*io estu mia-li finala decilita batlo",
        "wizu internacia-lo",
    ] {
        let ast = Node::parse(text).unwrap();
        println!("> {}\n```\n{:#?}\n```\n", ast.format(), ast.clone(),);
    }
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
        adj: Vec<Node>,
    },
    Verb {
        verb: Noun,
        adv: Vec<Node>,
        subj: Option<Box<Node>>,
        obj: Box<Node>,
    },
}

impl Node {
    fn parse(source: &str) -> Option<Self> {
        let source = source.trim();
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
                        result = result.trim().to_string() + "o";
                        return Some((Node::parse(&result), index + 1));
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
                let mut subj = String::new();
                let mut flag = false;
                let mut temp = String::new();
                while index < tokens.len() {
                    let current = tokens.get(index)?;
                    if let Some(verb) = current.strip_suffix(VERB) {
                        return Some(Node::Verb {
                            verb: Noun::parse(verb)?,
                            adv: advs,
                            subj: if subj.is_empty() {
                                None
                            } else {
                                Some(Box::new(Node::parse(subj.trim())?))
                            },
                            obj: Box::new(Node::parse(&get_after(index + 1)?)?),
                        });
                    } else if let Some(adv) = current.strip_suffix(ADV) {
                        advs.push(Node::parse(&([&temp.trim(), adv].join(SPACE) + "o"))?);
                        temp = String::new()
                    } else if current.ends_with(OBJ) {
                        subj = [&subj, current.to_owned()].join(SPACE);
                        flag = true
                    } else {
                        if flag {
                            temp.push_str(&(current.to_string() + SPACE));
                        } else {
                            subj.push_str(&(current.to_string() + SPACE));
                        }
                    }
                    index += 1
                }
            }

            if tokens.iter().any(|x| x.ends_with(OBJ)) {
                let (own, mut index) = get_owns(0).unwrap_or((None, 0));
                let mut adjs = vec![];
                let mut temp = String::new();
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
                        adjs.push(Node::parse(&([&temp.trim(), adj].join(SPACE) + "o"))?);
                        temp = String::new()
                    } else {
                        temp.push_str(&(current.to_string() + SPACE));
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
                    adv.iter()
                        .map(|x| {
                            let x = x.format();
                            if let Some(x) = x.strip_suffix(OBJ) {
                                x.to_string()
                            } else {
                                x
                            }
                        } + "e"
                            + SPACE)
                        .collect::<Vec<String>>()
                        .join(SPACE),
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
                    adj.iter()
                        .map(|x| {
                            let x = x.format();
                            if let Some(x) = x.strip_suffix(OBJ) {
                                x.to_string()
                            } else {
                                x
                            }
                        } + "a"
                            + SPACE)
                        .collect::<Vec<String>>()
                        .join(SPACE),
                    word.format(),
                )
            }
        }
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

#[derive(Clone, Debug)]
struct Vocabulary(String);
const BOCAS: [&str; 62] = [
    "d^", "c^", "d*i", "da*t", "mi", "yu", "est", "ed", "il", "av", "i-t", "hom", "a-l", "can",
    "izm", "ide", "liber", "soci", "naci", "anarki", "ru-n", "komp^u-t", "saiens", "program",
    "ekt", "ist", "wa-k", "act", "mov", "pros", "o-da", "prei", "raik", "lit", "aiz", "scir", "ne",
    "yes", "un", "on", "in", "ter", "eng", "de-t", "eny", "meny", "k^om", "ho-p", "teik", "los",
    "hav", "strong", "weak", "gu*d", "ba*d", "bes^", "mond", "batl", "final", "wiz", "raiz",
    "deci",
];

impl Vocabulary {
    fn parse(source: &str) -> Option<Self> {
        if BOCAS.contains(&source) {
            Some(Vocabulary(source.to_string()))
        } else {
            None
        }
    }

    fn format(&self) -> String {
        self.0.to_string()
    }
}
