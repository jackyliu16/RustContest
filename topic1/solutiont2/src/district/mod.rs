use serde_json::Map;
use std::cmp::max;
use std::fs;
use std::env;
use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use std::ops::{Add, Index};
use std::path::PathBuf;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{MapAccess, Visitor};
use serde_json::{json, Result, Value, Error};
use serde_with::serde_as;
use crate::district::union_find::DSU;
// REF:
// https://github.com/serde-rs/json/issues/652
// serde_with: version 2.0 changelog
    // * `tuple_list_as_map`: Use `BTreeMap` on a `Vec` of tuples:
    // ```rust
    // #[serde_as(as = "BTreeMap<_, _>")] // HashMap will also work
    // s: Vec<(i32, String)>,
    // ```
// https://tikv.github.io/doc/serde_with/rust/hashmap_as_tuple_list/index.html

mod union_find;

#[derive(Debug, Serialize, Deserialize)]
struct Data { // TODO: 这个地方不是很了解，可能可以通过什么方法避免那么抽象的引用
    // NOTE: 两种解决方法
    // 1. 在第一种方法中变量名必须要跟 key 相同，虽然来说对于本题而言，用 rename 修饰之后提供多个结构变量
    // 对于固定长度的文本也解决，但是太难看了
    // 2. 使用 ser(flatten) 将当前层次平铺成为 HashMap 然后再进行处理
    // #[serde(rename = "1")]
    // data: InnerData,
    #[serde(flatten)]
    data: HashMap<String, InnerData>,
}

#[derive(Debug, Serialize)]
struct InnerData(Vec<(String, Vec<String>)>);

impl<'de> Deserialize<'de> for InnerData {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        struct InnerDataVisitor;
        impl<'de> Visitor<'de> for InnerDataVisitor {
            type Value = InnerData;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("any valid JSON value even with duplicate key")
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>
            {
                // 在 visit 的时候根据自定义合并操作实现对应效果
                let mut conn:HashMap<String, Vec<String>> = HashMap::new();
                while let Some((key, value)) = map.next_entry()? {
                    conn.entry(key)
                        .or_insert_with(Vec::new)
                        .extend::<Vec<String>>(value);
                }
                let res: Vec<(String, Vec<String>)> = conn.into_iter().collect::<Vec<(String, Vec<String>)>>();
                Ok(InnerData(res))
            }
        }
        Ok(deserializer.deserialize_map(InnerDataVisitor)?)
    }
}

pub fn count_provinces() -> String {
    let manifest_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = PathBuf::from(manifest_path).join("district.json");

    let data = fs::read_to_string(&path).unwrap();
    let json_data: Data = serde_json::from_str(&data).unwrap();

    // 获取将对应的内容解析并写入到数据结构
    let mut turns: Vec<HashMap<&str, Vec<&str>>> = vec![HashMap::new(); json_data.data.len()];  // 由于迭代器，第一个为空
    for (idx, turn) in turns.iter_mut().enumerate() {
        if let Some(data) = json_data.data.get((idx + 1).to_string().as_str()) {      // 由于迭代器，第一个为空
            // struct InnerData(Vec<(String, Vec<String>)>);
            for (k, v) in data.0.iter() {
                turn.insert(k.as_str(),v.iter().map(|x| x.as_str()).collect::<Vec<&str>>());
            }
        }
    }

    dbg!(&turns);
    turns.iter()
        .map(|turn| count_connected_components(turn).to_string())
        .collect::<Vec<String>>()
        .join(",")
}

/// 计算无向图的连通块个数
fn count_connected_components<'a>(graph: &'a HashMap<&'a str, Vec<&'a str>>) -> usize {
    /// 获取 str 的一个 Vec, 其中每一个的 idx 代指对应的节点
    let node = generate_idx_str_mapping(graph);
    let mut union_find = DSU::with_capacity(node.len());    
    
    for &key in graph.keys() {
        for &val in graph.get(key).unwrap() {
            union_find.union(get_idx(key, &node), get_idx(val, &node));
        }
    }
    union_find.count_sets()
}

/// 将原先 HashMap<&str, Vec<&str>> 的数据中出现的每一个城市唯一性的插入到 Vec<&str> 中进行存储
/// 在对应数据被正确插入到 Vec<&str> 中之后, idx <-> &str 的映射关系被同样保存在该数据结构中
fn generate_idx_str_mapping<'a>(graph: &'a HashMap<&str, Vec<&str>>) -> Vec<&'a str> {
    let mut node = vec![];
    for &key in graph.keys() {
        insert_data(key, &mut node);
        graph.get(key).unwrap().iter().for_each(|&x| insert_data(x, &mut node));
    }
    node
}

/// 根据预定义的唯一性插入原则, 获取对应 &str 到 idx 的映射
fn get_idx<'a>(data: &'a str, vec: &Vec<&'a str>) -> usize {
    vec.iter().position(|&s| s == data).unwrap()
}

/// 根据需求 **唯一性** 的插入数据到 Vec<&'a str>, 以存储 idx->&str 的映射信息
fn insert_data<'a>(data: &'a str, vec: &mut Vec<&'a str>) {
    if ! vec.contains(&data) {
        vec.push(data);
    }
}

/// 将所有临接点标记为访问并递归进入
#[deprecated]
fn dfs<'a>(node: &'a str, graph: &'a HashMap<&'a str, Vec<&'a str>>, visited: &mut HashSet<&'a str>) {
    visited.insert(node);
    println!("visited: {node}");

    if let Some(neighbors) = graph.get(node) {
        for &neighbor in neighbors {
            if !visited.contains(neighbor) {
                dfs(neighbor, graph, visited);
            }
        }
    }
}
