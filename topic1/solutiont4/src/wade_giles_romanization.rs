use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use unicode_normalization::UnicodeNormalization;

// TODO: 可以将对应威妥玛拼音的映射关系与前面字形到拼音的转换表结合在一起,然后通过写入到某个特定的文件,用时再读取的方式降低重复构建的性能损耗
pub fn converter(input: &str) -> String {
    dbg!(&input);
    let kMandarin_map = load_pin_yin_file("kMandarin.txt".into()).unwrap();
    let mapping = load_mapping("mapping_pingyin_to_wade_giles".into()).unwrap();
    // dbg!(&kMandarin_map);

    // 首先基于基础字形映射 将对应的中文内容转换成为对应的 拼音
    let pingyin = input.chars()
        // .map(|c| kMandarin_map[&c].clone())
        .map(|c| kMandarin_map.get(&c).unwrap_or(&" ".into()).clone())
        .collect::<Vec<_>>();
    // dbg!(&pingyin);

    if pingyin.len() <= 1 && pingyin[0] == " " {
        return "".into()
    }

    // 然后再根据拼音与威妥玛拼音之间的映射关系, 将代表对应字符的拼音转换成为对应的威妥玛拼音
    // ("诸葛亮", "Chu ko liang"),
    // stage 1: zhu ge liang
    let mut res: Vec<_> = pingyin
        .iter()
        .map(|words| mapping.get(words).unwrap_or(&" ".into()).clone())
        .collect();

    // 根据答案的要求,将字符串的第一个 ASCII 转换成为大写表达形式
    if let Some(first) = res.first_mut() {
        let mut chars = first.chars();
        match chars.next() {
            None => (),
            Some(c) => {
                *first = std::iter::once(c.to_uppercase().next().unwrap())
                    .chain(chars)
                    .collect()
            }
        }
    }

    res.join(" ")
}

/// 从基于 CARGO_MANIFEST_DIR 的相对路径中加载 file_name 的文件
/// 并根据文件内部的组织规则, 将每行中的一一映射转换成为对应的 HashMap 映射
pub fn load_mapping<'a>(file_name: String) -> io::Result<HashMap<String, String>> {
    let manifest_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = PathBuf::from(manifest_path).join(file_name);
    let mut map = HashMap::new();

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("#") {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        let modify = parts[0].replace("`", "'").to_string();
        if parts.len() >= 2 {
            map.insert(parts[1].to_string(), modify);
        }
    }

    Ok(map)
}

/// 从基于 CARGO_MANIFEST_DIR 的相对路径中加载 file_name 的拼音文件
/// 并根据文件内部的组织规则, 将每行的多个对应关系转换成为对应的 HashMap 映射
pub fn load_pin_yin_file(file_name: String) -> io::Result<HashMap<char, String>> {
    let manifest_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = PathBuf::from(manifest_path).join(file_name);
    let mut map: HashMap<char, String> = HashMap::new();
    let file = File::open(path)?;

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("#") {
            continue
        }
        let line = line.split_whitespace().collect::<Vec<_>>();
        // U+4CA2: téng  # 䲢
        // U+2CE2A: fán  # 𬸪  => U+9DED
        // dbg!(&line);
        let first = line[0].strip_suffix(":").unwrap();     // U+4CA2
        let second = line[1]                                    // fán -> fan
            .to_string()
            .nfkd()
            .filter(|c| c.is_ascii())
            .collect::<String>();

        map.insert(unicode_to_char(&first).unwrap(), second.clone());
        // 对于一行中存在多个映射结果的情况而言, 构建 yǐ 到不同的解释字形之间的相关关系
        // U+2CE88: yǐ  # 𬺈  =>  U+9F6E
        if line.len() >= 6 {
            if let Some(val) = unicode_to_char(line[5]) {
                map.insert(val, second);
            } else {
                // dbg!(line);
            }
        }
    }


    Ok(map)
}

fn unicode_to_char(unicode_str: &str) -> Option<char> {
    if let Some(hex) = unicode_str.strip_prefix("U+") {
        u32::from_str_radix(hex, 16).ok().and_then(std::char::from_u32)
    } else {
        None
    }
}