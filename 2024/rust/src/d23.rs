use std::{
    collections::{HashMap, HashSet},
    vec,
};

use itertools::Itertools;

fn parse(input: &str) -> HashMap<String, HashSet<String>> {
    let mut map = HashMap::new();
    for line in input.trim().lines() {
        let mut connection = line.trim().split('-');
        let a = connection.next().unwrap().to_string();
        let b = connection.next().unwrap().to_string();
        map.entry(a.clone())
            .or_insert(HashSet::new())
            .insert(b.clone());
        map.entry(b.clone())
            .or_insert(HashSet::new())
            .insert(a.clone());
    }
    map
}

pub fn part1(input: &str) -> String {
    let connections = parse(input);

    connections
        .keys()
        .filter(|k| k.starts_with("t"))
        .flat_map(|k| {
            let neighbors = connections.get(k).unwrap();
            neighbors
                .iter()
                .combinations(2)
                .filter(|nodes| {
                    let a = nodes[0];
                    let b = nodes[1];
                    connections.get(a).unwrap().contains(b)
                })
                .map(move |nodes| {
                    let mut group = vec![k];
                    group.extend(nodes);
                    group.sort();
                    group
                })
        })
        .unique()
        .count()
        .to_string()
}

fn groups(connections: &HashMap<String, HashSet<String>>) -> Vec<HashSet<String>> {
    let mut groups = connections
        .keys()
        .map(|k| HashSet::from([k.to_string()]))
        .collect_vec();
    let mut found = true;
    while found {
        found = false;
        for (node, neighbors) in connections.iter() {
            for group in groups.iter_mut() {
                if neighbors.is_superset(group) {
                    let inserted = group.insert(node.clone());
                    if !found {
                        found = inserted;
                    }
                }
            }
        }
    }
    groups
}

pub fn part2(input: &str) -> String {
    let connections: HashMap<String, HashSet<String>> = parse(input);

    groups(&connections)
        .iter()
        .max_by_key(|group| group.len())
        .map(|group| {
            let mut group = group.iter().cloned().collect_vec();
            group.sort();
            group
        })
        .unwrap()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
        kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn
        ";
        assert_eq!(part1(input), "7");
    }

    #[test]
    fn test_part2() {
        let input = "
        kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn
        ";
        assert_eq!(part2(input), "co,de,ka,ta");
    }
}
