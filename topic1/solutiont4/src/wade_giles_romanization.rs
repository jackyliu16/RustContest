use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::pin::pin;
use unicode_normalization::UnicodeNormalization;

pub fn converter(input: &str) -> String {
    dbg!(&input);
    let kMandarin_map = load_pingyin_file("kMandarin.txt".into()).unwrap();
    let mapping = load_mapping("mapping_pingyin_to_wade_giles".into()).unwrap();

    // dbg!(&kMandarin_map);

    let pingyin = input.chars()
        // .map(|c| kMandarin_map[&c].clone())
        .map(|c| kMandarin_map.get(&c).unwrap_or(&" ".into()).clone())
        .collect::<Vec<_>>();

    // dbg!(&pingyin);

    if pingyin.len() <= 1 && pingyin[0] == " " {
        return "".into()
    }

    // ("诸葛亮", "Chu ko liang"),
    // stage 1: zhu ge liang
    let mut res: Vec<_> = pingyin
        .iter()
        .map(|words| mapping.get(words).unwrap_or(&" ".into()).clone())
        .collect();

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

pub fn load_pingyin_file(file_name: String) -> io::Result<HashMap<char, String>> {
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