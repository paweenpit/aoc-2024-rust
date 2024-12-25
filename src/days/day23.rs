use itertools::{iproduct, Itertools};
use std::collections::{HashMap, HashSet};

pub fn main() {
    let input = std::fs::read_to_string("inputs/day23.input").expect("Can't read file");
    let map: HashMap<&str, HashSet<&str>> = input.lines().fold(HashMap::new(), |mut map, line| {
        let parts = line.split_once("-").unwrap();
        map.entry(parts.0).or_insert(HashSet::new()).insert(parts.1);
        map.entry(parts.1).or_insert(HashSet::new()).insert(parts.0);
        map
    });

    // part 1
    let nodes = map.keys().cloned().collect::<Vec<&str>>();
    // find 3 connected nodes
    let connect3: Vec<[usize; 3]> = iproduct!(0..nodes.len(), 0..nodes.len(), 0..nodes.len())
        .map(|(a, b, c)| [a, b, c])
        .filter(|[a, b, c]| {
            let na = nodes[*a];
            let nb = nodes[*b];
            let nc = nodes[*c];

            a < b
                && b < c
                && na != nb
                && na != nc
                && nb != nc
                && map[na].contains(nb)
                && map[nb].contains(nc)
                && map[nc].contains(na)
                && (na.starts_with('t') || nb.starts_with('t') || nc.starts_with('t'))
        })
        .collect();
    println!("Part 1: {}", connect3.len());

    // part 2
    // find largest clique in graph
    // https://www.geeksforgeeks.org/maximal-clique-problem-recursive-solution/
    let graph: Graph = map
        .iter()
        .map(|(k, v)| {
            let set: HashSet<String> = v.iter().map(|s| s.to_string()).collect();
            (k.to_string(), set)
        })
        .collect();
    let mut cliques: Vec<HashSet<String>> = Vec::new();
    let r = HashSet::new();
    let p: HashSet<String> = graph.keys().cloned().collect();
    let x = HashSet::new();

    bron_kerbosch(r, p, x, &graph, &mut cliques);
    let x: String = cliques
        .into_iter()
        .max_by_key(|v| v.len())
        .unwrap()
        .into_iter()
        .sorted()
        .join(",");
    println!("Part 2: {:?}", x);
}

// Define a type alias for the graph structure
type Graph = HashMap<String, HashSet<String>>;

fn bron_kerbosch(
    r: HashSet<String>,
    mut p: HashSet<String>,
    mut x: HashSet<String>,
    graph: &Graph,
    cliques: &mut Vec<HashSet<String>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
        return;
    }
    let mut p_iter = p.clone();
    for v in p_iter.drain() {
        let mut r_union_v = r.clone();
        r_union_v.insert(v.clone());

        let p_intersection: HashSet<String> = p.intersection(&graph[&v]).cloned().collect();
        let x_intersection: HashSet<String> = x.intersection(&graph[&v]).cloned().collect();

        bron_kerbosch(r_union_v, p_intersection, x_intersection, graph, cliques);

        p.remove(&v);
        x.insert(v);
    }
}
