use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;
use std::str::FromStr;

pub fn converter(input: &str, tp: &str) -> String {
    let map = get_map_from_file(TranslationType::from_str(tp).unwrap()).unwrap();
    let mut out: Vec<char> = Vec::new();
    for c in input.chars() {
       out.push(map.get(&c.to_string()).unwrap_or(&c.to_string()).parse().unwrap());
    }
    out.iter().collect()
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

fn get_map_from_file(types: TranslationType) -> io::Result<HashMap<String, String>> {
    let file = match types {
        TranslationType::Simple2Traditional => { "STCharacters.txt" },
        TranslationType::Traditional2Simple => { "TSCharacters.txt" },
    };
    let manifest_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = PathBuf::from(manifest_path).join("dict").join(file);
    dbg!(&path);

    let mut map: HashMap<String, String> = HashMap::new();
    let mut file = File::open(path)?;

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let line = line.split_whitespace().collect::<Vec<&str>>();
        let s = line[0];
        let t = if line.len() == 3 {
            line[2]
        } else {
            line[1]
        };
        match types {
            TranslationType::Traditional2Simple => { map.insert(t.to_string(), s.to_string()); }
            TranslationType::Simple2Traditional => { map.insert(s.to_string(), t.to_string()); }
        }
    }

    Ok(map)
}
