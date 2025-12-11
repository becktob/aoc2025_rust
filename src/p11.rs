use std::collections::{HashMap, HashSet};

pub fn solve(part2: bool) -> String {
    let input = std::fs::read_to_string("input_10.txt").expect("could not read file");
    if part2 {
        "WIP".to_string()
        //solve_2(&input).to_string()
    } else {
        "WIP".to_string()
        //solve_1(&input).to_string()
    }
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
