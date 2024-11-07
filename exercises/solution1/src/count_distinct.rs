pub fn new_count_distinct(input_str: &str) -> usize {
    input_str
        .split(',')
        .map(|c| c.trim())
        .filter(|c| !c.is_empty())
        .collect::<std::collections::HashSet<&str>>()
        .len()
}