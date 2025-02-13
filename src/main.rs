use suffix::{ADJ, ADV, OBJ, OWN, VERB};

fn main() {
    println!("# Komona Lange-zi prosactisto\n");
    for text in [
        "c^u yuo estu i-tcana homa-lo",
        "d*ii komp^u-tekta programengo prosactu menya de-to",
        "mio stronge k^omavu internacia-la antiwa-ri movesto inu bes^mondo",
        "finale deciedu joinu libero-straika anarkiizmi bunto wizu mii frendo",
    ] {
        let ast = Node::parse(text).unwrap();
        println!("> {}\n```\n{:?}\n```\n", ast.format(), ast.clone());
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
const BOCAS: [&str; 78] = [
    "d^",       // 命令
    "c^",       // 疑問
    "d*i",      // これ
    "da*t",     // それ
    "mi",       // 私
    "yu",       // あなた
    "hi-",      // 彼
    "est",      // 説明
    "ed",       // 過去
    "il",       // 未来
    "av",       // 現在
    "i-t",      // 食べる
    "spi-k",    // 言う/喋る
    "a:ud",     // 聞く
    "lu*k",     // 見る
    "sve-t",    // 光
    "da-k",     // 闇
    "hom",      // 人間
    "a-l",      // 集合
    "can",      // 可能
    "izm",      // 主義/思想
    "ist",      // もの(主体)
    "ide",      // 概念
    "liber",    // 自由
    "soci",     // 社会
    "naci",     // 国家
    "blast",    // 権力
    "anarki",   // 無政府
    "ru-n",     // 走る
    "komp^u-t", // 計算
    "saiens",   // 科学
    "program",  // プログラム
    "ekt",      // 機械
    "wa-k",     // 仕事
    "act",      // する(行為)
    "mov",      // 動き
    "pros",     // 処理
    "o-da",     // 命令
    "prei",     // 遊び
    "raik",     // 比喩
    "lit",      // 性質
    "aiz",      // 変化
    "scir",     // させる
    "ne",       // 否定
    "anti",     // 反対
    "yes",      // 肯定
    "un",       // 無い
    "on",       // 有る
    "in",       // 中に
    "ter",      // 越え
    "eng",      // すること
    "de-t",     // データ
    "eny",      // 何か
    "meny",     // 複数の
    "k^om",     // 興味
    "ho-p",     // 希望
    "teik",     // 取得
    "los",      // 失う
    "hav",      // 持つ
    "o-st",     // 最上級
    "e-r",      // 比較級
    "strong",   // 強い
    "weak",     // 弱い
    "gu*d",     // 良い
    "ba*d",     // 悪い
    "bes^",     // 全て
    "mond",     // 世界
    "batl",     // 戦い
    "wa-r",     // 戦争
    "final",    // 最後
    "wiz",      // 共に
    "stand",    // 立つ
    "deci",     // 決定
    "frend",    // 友達
    "rela",     // 関係
    "emo-t",    // 心
    "bunt",     // 同盟
    "join",     // 参加
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
