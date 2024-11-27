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
        let mut ts_phrases_map = get_map_from_file(TranslationType::Traditional2Simple, "TSPhrases.txt".into()).unwrap();
        if let Some(value) = ts_phrases_map.get(input.into()) {
            println!("directly");
            return value.clone();
        }

        let char_map = get_map_from_file(TranslationType::Simple2Traditional, "TSCharacters.txt".into()).unwrap();
        return input
            .to_string()
            .chars()
            .map(|c| { char_map.get(&c.to_string()).unwrap_or(&c.to_string()).clone() })
            .collect::<String>()
    } else {
        dbg!("s2t");
        let mut phrases_map = get_map_from_file(TranslationType::Simple2Traditional, "TSPhrases.txt".into()).unwrap();
        let ts_phrases_map = get_map_from_file(TranslationType::Simple2Traditional, "STPhrases.txt".into()).unwrap();
        for (k, v) in ts_phrases_map { // TODO Combine
            phrases_map.insert(k.into(), v.into());
        }

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

        return input
            .to_string()
            .chars()
            .map(|c| { hk_char_map.get(&c.to_string()).unwrap_or(&c.to_string()).clone()})
            // .map(|c| { println!("{}", &c); c})
            // .map(|c| { tw_char_map.get(&c).unwrap_or(&c).clone() })
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
