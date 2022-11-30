use crate::utils::{get_input, InputType};
use std::{
    collections::{HashMap, HashSet},
    io,
};

pub fn main() -> io::Result<()> {
    println!(
        "part one: {}",
        part_one(&get_input(2021, 12, InputType::Challenge, 0)?)
    );
    println!(
        "part two: {}",
        part_two(&get_input(2021, 12, InputType::Challenge, 0)?)
    );
    Ok(())
}

fn is_lowercase(string: &String) -> bool {
    string.chars().all(|letter| letter >= 'a' && letter <= 'z')
}

fn build_map(input: &str) -> HashMap<String, HashSet<String>> {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();
    for line in input.lines() {
        let mut node_names = line.split('-');
        if let (Some(node_a), Some(node_b)) = (node_names.next(), node_names.next()) {
            for (root, leaf) in [(node_a, node_b), (node_b, node_a)] {
                let (root_string, leaf_string) = (root.to_string(), leaf.to_string());

                if let Some(connection_set) = map.get_mut(&root_string) {
                    connection_set.insert(leaf_string.clone());
                } else {
                    let mut connection_set: HashSet<String> = HashSet::new();
                    connection_set.insert(leaf_string.clone());
                    map.insert(root_string.clone(), connection_set);
                }
            }
        }
    }

    map
}

fn get_path_count(
    map: &HashMap<String, HashSet<String>>,
    current: String,
    mut visited: HashSet<String>,
    can_visit_twice: bool,
) -> i32 {
    if current == "end".to_string() {
        return 1;
    }

    if is_lowercase(&current) {
        visited.insert(current.to_string());
    }

    if let Some(neighbors) = map.get(&current) {
        let nodes_to_check = neighbors.difference(&visited);

        let mut paths = 0;

        if can_visit_twice {
            let nodes_to_visit_twice = neighbors
                .intersection(&visited)
                .filter(|name| **name != "start".to_string());

            paths += nodes_to_visit_twice
                .map(|node| -> i32 { get_path_count(map, node.clone(), visited.clone(), false) })
                .sum::<i32>();
        }

        return paths
            + nodes_to_check
                .map(|node| -> i32 {
                    get_path_count(map, node.clone(), visited.clone(), can_visit_twice)
                })
                .sum::<i32>();
    }
    panic!(
        "Node <{}> has no neighbors... how did we get here?",
        current
    );
}

fn part_one(input: &str) -> i32 {
    let map = build_map(input);
    let path_count = get_path_count(&map, "start".to_string(), HashSet::new(), false);
    path_count
}

fn part_two(input: &str) -> i32 {
    let map = build_map(input);
    let path_count = get_path_count(&map, "start".to_string(), HashSet::new(), true);
    path_count
}
