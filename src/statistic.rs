use std::collections::HashMap;

pub fn top_command(commands: &[String], limit: usize) -> Vec<(String, usize)> {
    let mut counts = HashMap::new();
    for cmd in commands {
        *counts.entry(cmd.clone()).or_insert(0) += 1;
    }
    let mut sorted: Vec<_> = counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted.truncate(limit);
    sorted
}

// Аналог most_used_utils
pub fn most_used_utils(raw_commands: &[String], limit: usize, aliases: Option<&[String]>) -> Vec<(String, usize)> {
    let mut utils = Vec::new();
    for cmd in raw_commands {
        let utility = cmd.split_whitespace().next().unwrap_or("").to_string();
        if let Some(alias_list) = aliases {
            if alias_list.contains(&utility) {
                utils.push(utility);
            }
        } else {
            utils.push(utility);
        }
    }
    top_command(&utils, limit)
}