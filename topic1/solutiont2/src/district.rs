use std::cmp::max;
use std::fs;
use std::env;
use std::collections::{HashMap, HashSet};
use std::ops::Add;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json::{json, Result, Value};

pub fn count_provinces() -> String {
    let path = env::current_dir()
        .expect("Cannot access current working directory")
        .parent()
        .expect("Cannot get parent directory of current working directory")
        .join("solutiont2")
        .join("district.json");

    println!("Provinces: {}", path.display());

    let data = fs::read_to_string(&path).unwrap();
    // let data = fs::read_to_string("topic1/solutiont2/district.json").unwrap();
    println!("{:?}", env::current_dir());

    let json_data: Value = serde_json::from_str(&data).unwrap();

    // 获取总 turns 数量
    let mut num_turns = 5;
    if let Value::Object(map) = &json_data {
        num_turns = map.keys()
            .filter_map(|k| k.parse::<usize>().ok())
            .max().unwrap_or(5)
    };

    // 获取将对应的内容解析并写入到数据结构
    let mut turns: Vec<HashMap<&str, Vec<&str>>> = vec![HashMap::new(); num_turns]; // 由于迭代器，第一个为空
    for (idx, mut turn) in turns.iter_mut().enumerate() {
        if let Some(data) = json_data.get((idx + 1).to_string().as_str()) {      // 由于迭代器，第一个为空
            if let Some(map) = data.as_object() {
                for (k, v) in map {
                    turn.insert(k.as_str(), v.as_array().unwrap().iter()
                        .map(|x| x.as_str()
                            .unwrap())
                        .collect());
                }
            }
        }
    }

    turns.iter()
        .map(|turn| count_connected_components(turn).to_string())
        .collect::<Vec<String>>()
        .join(",")
}

/// 计算无向图的连通块个数
fn count_connected_components<'a>(graph: &'a HashMap<&'a str, Vec<&'a str>>) -> usize {
    let mut visited: HashSet<&'a str> = HashSet::new();
    let mut count = 0;
    for &node in graph.keys() {
        if !visited.contains(node) {
            count += 1;
            dfs(node, graph, &mut visited);
        }
    }
    count
}

/// 将所有临接点标记为访问并递归进入
fn dfs<'a>(node: &'a str, graph: &'a HashMap<&'a str, Vec<&'a str>>, visited: &mut HashSet<&'a str>) {
    visited.insert(node);

    if let Some(neighbors) = graph.get(node) {
        for &neighbor in neighbors {
            if !visited.contains(neighbor) {
                dfs(neighbor, graph, visited);
            }
        }
    }
}