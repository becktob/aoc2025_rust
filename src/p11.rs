use std::collections::{HashMap, HashSet};

pub fn solve(part2: bool) -> String {
    let input = std::fs::read_to_string("input_11.txt").expect("could not read file");
    if part2 {
        "WIP".to_string()
        //solve_2(&input).to_string()
    } else {
        solve_1(&input).to_string()
    }
}

fn solve_1(input: &str) -> usize {
    let devices = parse(input);
    // paths_to_out_grow(&devices).get("you").unwrap().len() // too slow.
    paths_to_out(&devices, "you").len()
}

fn solve_2(input: &str) -> usize {
    let devices = parse(input);

    paths(&devices, "svr", "fft").len()
        * paths(&devices, "fft", "dac").len()
        * paths(&devices, "dac", "out").len()
}

struct Node {
    label: String,
    to: HashSet<String>,
    from: HashSet<String>,
}

fn empty_node(label: &str) -> Node {
    Node {
        label: label.to_string(),
        from: HashSet::new(),
        to: HashSet::new(),
    }
}

type Devices = HashMap<String, Node>;

fn parse(input: &'_ str) -> Devices {
    let mut devices_vec: Vec<_> = input
        .lines()
        .map(|l| l.split_once(':').unwrap())
        .map(|(a, _)| (a.to_string(), empty_node(a)))
        .collect();
    devices_vec.push(("out".to_string(), empty_node("out")));
    let mut devices = Devices::from_iter(devices_vec);

    input
        .lines()
        .map(|l| l.split_once(':').unwrap())
        .for_each(|(a, b)| {
            b.trim().split(" ").for_each(|b| {
                let from_name = devices.get(a).unwrap().label.to_string();
                let to_name = devices.get(b).unwrap().label.to_string();
                devices.get_mut(a).unwrap().to.insert(to_name);
                devices.get_mut(b).unwrap().from.insert(from_name);
            });
        });

    devices
}

#[cfg(test)]
fn paths_to_out_grow(devices: &Devices) -> HashMap<String, HashSet<Vec<String>>> {
    let mut paths_to_out = HashMap::new();
    paths_to_out.insert(
        "out".to_string(),
        HashSet::from_iter([vec!["out".to_string()]]),
    );

    let mut nodes_todo = HashSet::new();
    nodes_todo.insert("out".to_string());

    while !nodes_todo.is_empty() {
        let node_name = nodes_todo.iter().cloned().next().unwrap();
        nodes_todo.remove(&node_name);

        devices
            .get(&node_name)
            .unwrap()
            .from
            .iter()
            .for_each(|node_leading_here| {
                let paths_from_here_to_out = paths_to_out.get(&node_name).unwrap().clone();
                paths_from_here_to_out
                    .iter()
                    .for_each(|path_from_here: &Vec<String>| {
                        let mut path_from_node = path_from_here.clone();
                        path_from_node.insert(0, node_leading_here.to_string());
                        paths_to_out
                            .entry(node_leading_here.to_string())
                            .or_insert(HashSet::new())
                            .insert(path_from_node);

                        // process that node (again?)
                        nodes_todo.insert(node_leading_here.to_string());
                    })
            })
    }

    paths_to_out
}

fn paths_to_out(devices: &Devices, label: &str) -> Vec<Vec<String>> {
    paths(devices, label, "out")
}
fn paths(devices: &Devices, from: &str, target: &str) -> Vec<Vec<String>> {
    if from == target {
        return [[target.to_string()].to_vec()].to_vec();
    }

    devices
        .get(from)
        .unwrap()
        .to
        .iter()
        .flat_map(|to| paths(devices, devices.get(to).unwrap().label.as_str(), target))
        .map(move |mut remaining_path| {
            remaining_path.insert(0, from.to_string());
            remaining_path.clone()
        })
        .collect()
}

#[cfg(test)]
static EXAMPLE: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

#[cfg(test)]
static EXAMPLE_2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

#[test]
fn test_parse() {
    let devices = parse(&EXAMPLE);
    assert_eq!(devices.len(), 10 + 1);
    assert!(devices.contains_key("you"));
    assert!(devices.contains_key("out"));
    assert_eq!(
        devices["ccc"].to,
        HashSet::from(["fff", "eee", "ddd"].map(str::to_string))
    );
    assert_eq!(
        devices["out"].from,
        HashSet::from(["eee", "fff", "ggg", "iii"].map(str::to_string))
    );
}

#[test]
fn test_paths_to_out_grow_you() {
    let devices = parse(&EXAMPLE);
    let paths = paths_to_out_grow(&devices);
    let paths_to_you = paths.get("you").unwrap();
    assert_eq!(paths_to_you.len(), 5);
    let known_path = ["you", "ccc", "fff", "out"].map(str::to_string).to_vec();
    assert!(paths_to_you.contains(&known_path));
}

#[test]
fn test_paths_to_out_grow_eee() {
    let devices = parse(&EXAMPLE);
    let paths = paths_to_out_grow(&devices);
    let paths_to = paths.get("eee").unwrap();
    assert_eq!(paths_to.len(), 1);
    let only_path = ["eee", "out"].map(str::to_string).to_vec();
    assert_eq!(*paths_to, HashSet::from_iter([only_path]));
}

#[test]
fn test_paths_to_out_grow_bbb() {
    let devices = parse(&EXAMPLE);
    let paths = paths_to_out_grow(&devices);
    let paths_to = paths.get("bbb").unwrap();
    assert_eq!(paths_to.len(), 2);
    let known_paths = [
        ["bbb", "eee", "out"].map(str::to_string).to_vec(),
        ["bbb", "ddd", "ggg", "out"].map(str::to_string).to_vec(),
    ];
    assert_eq!(*paths_to, HashSet::from_iter(known_paths));
}

#[test]
fn test_paths_to_out() {
    let devices = parse(&EXAMPLE);
    let paths_to_out = paths_to_out(&devices, "you");
    assert_eq!(paths_to_out.len(), 5);
    let known_path = ["you", "ccc", "fff", "out"].map(str::to_string).to_vec();
    assert!(paths_to_out.contains(&known_path));
}

#[test]
fn test_solve_1_example() {
    assert_eq!(solve_1(EXAMPLE), 5);
}

#[test]
fn test_solve_1() {
    assert_eq!(solve(false), "423");
}

#[test]
fn test_solve_2_example() {
    assert_eq!(solve_2(EXAMPLE_2), 2);
}
