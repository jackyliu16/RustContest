use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;
use std::str::FromStr;

pub fn converter(input: &str, tp: &str) -> String {
    dbg!(input);
    if tp == "t2s" {
        dbg!("t2s");
        // 从文件中获取 繁体到简体 的 词映射表
        // 由于当前测试用例的体现更加倾向于台湾正体中文, 繁体到简体的转换是绝对的一一对应
        let mut ts_phrases_map = get_map_from_file(TranslationType::Traditional2Simple, "TSPhrases.txt".into()).unwrap();
        if let Some(value) = ts_phrases_map.get(input.into()) {
            println!("directly");
            return value.clone();
        }

        // 从文件中获取 繁体到简体的 字映射表
        // 如果满足的话直接显示对应到 简体呈现, 如果不存在对应的映射, 则之间返回原来的那个字符
        let char_map = get_map_from_file(TranslationType::Simple2Traditional, "TSCharacters.txt".into()).unwrap();
        return input
            .to_string()
            .chars()
            .map(|c| { char_map.get(&c.to_string()).unwrap_or(&c.to_string()).clone() })
            .collect::<String>()
    } else {
        dbg!("s2t");
        // 从文件中获取 简体到繁体的 词映射表
        // 由于在 OpenCC 导出的数据库中 简体转换为繁体 的数据库 与 繁体转换为简体的数据库不是严格等价的
        // 因此此处添加了一个步骤将二者反方向链接在一起以拓展词库
        let mut phrases_map = get_map_from_file(TranslationType::Simple2Traditional, "TSPhrases.txt".into()).unwrap();
        let ts_phrases_map = get_map_from_file(TranslationType::Simple2Traditional, "STPhrases.txt".into()).unwrap();
        for (k, v) in ts_phrases_map { // TODO Combine
            phrases_map.insert(k.into(), v.into());
        }

        // 根据对于测试样例的分析, 测试样例使用的数据法有部分 字符 存在于 台湾正体 中
        // 因此在进行 简体到繁体 字符映射表时, 需要先根据台湾正体 额外转换表对于部分字符进行映射
        let hk_char_map = get_map_from_file(TranslationType::Simple2Traditional, "STCharacters.txt".into()).unwrap();
        let tw_char_map = get_map_from_file(TranslationType::Simple2Traditional, "TWVariants.txt".into()).unwrap();
        if let Some(value) =  phrases_map.get(input.into()) {
            println!("directly");
            return value
                .clone()
                .chars()
                .map(|c| tw_char_map.get(&c.to_string()).unwrap_or(&c.to_string()).clone())
                .collect::<String>()
        }

        // 然后再进行逐字符 HK 繁体映射
        return input
            .to_string()
            .chars()
            .map(|c| { hk_char_map.get(&c.to_string()).unwrap_or(&c.to_string()).clone()})
            .collect::<String>()
    }
    String::new()
}

enum TranslationType {
    Simple2Traditional,
    Traditional2Simple,
}

impl FromStr for TranslationType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "t2s" => Ok(TranslationType::Traditional2Simple),
            _ => Ok(TranslationType::Simple2Traditional),
        }
    }
}

/// 根据传入的 types 所指示的繁简体转换方式获取对应的 HashMap 映射
/// 其中读取的内容
fn get_map_from_file(types: TranslationType, path: String) -> io::Result<HashMap<String, String>> {
    let manifest_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = PathBuf::from(manifest_path).join("dict").join(path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut map = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let line = line.split_whitespace().collect::<Vec<&str>>();
        match types {
            TranslationType::Simple2Traditional => {
                map.insert(line[0].to_string(), line[1].to_string());
            },
            TranslationType::Traditional2Simple => {
                map.insert(line[1].to_string(), line[0].to_string());
            }
        }
    }
    Ok(map)
}
