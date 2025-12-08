use std::collections::HashSet;

pub fn solve(part2: bool) -> String {
    let input = std::fs::read_to_string("input_07.txt").expect("could not read file");
    if part2 {
        "WIP".to_string()
        // crate::p07::solve_2(&input).to_string()
    } else {
        "WIP".to_string()
        //crate::p07::solve_1(&input).to_string()
    }
}

fn solve_1(input: &str, n_to_connect: usize) -> usize {
    let boxes = parse_boxes(input);

    let circuits = connect_closest(&boxes, n_to_connect);
    let mut circuit_sizes: Vec<_> = circuits.iter().map(HashSet::len).collect();
    circuit_sizes.sort();
    circuit_sizes.reverse();

    circuit_sizes[0..3].iter().product()
}

type Box = [i64; 3];
type Circuit = HashSet<Box>;

fn distance(first: &Box, other: &Box) -> f64 {
    f64::sqrt(
        ((first[0] - other[0]).pow(2) + (first[1] - other[1]).pow(2) + (first[2] - other[2]).pow(2))
            as f64,
    )
}

fn parse_boxes(input: &str) -> Vec<Box> {
    input
        .lines()
        .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect())
        .map(|xyz: Vec<i64>| [xyz[0], xyz[1], xyz[2]])
        .collect()
}

fn sorted_distances(boxes: &Vec<Box>) -> Vec<(&Box, &Box, f64)> {
    let mut distances = boxes
        .iter()
        .enumerate()
        .flat_map(|(i, a)| boxes[i + 1..].iter().map(move |b| (a, b, distance(a, b))))
        .collect::<Vec<(&Box, &Box, f64)>>();
    distances.sort_by(|(_, _, d1), (_, _, d2)| d1.total_cmp(d2));
    distances
}

fn connect_closest(boxes: &Vec<Box>, n_to_connect: usize) -> Vec<Circuit> {
    let sorted_by_distance = sorted_distances(boxes);

    let mut circuits: Vec<Circuit> = vec![];
    sorted_by_distance
        .iter()
        .take(n_to_connect)
        .for_each(|(a, b, _)| {
            // todo: if let possible with two Options?
            let idx_a = circuits.iter().position(|c| c.contains(*a));
            let idx_b = circuits.iter().position(|c| c.contains(*b));

            if let Some(idx_a) = idx_a
                && let Some(idx_b) = idx_b
                && idx_a != idx_b
            {
                let circ_a = circuits.swap_remove(idx_a);
                let circ_b = circuits.swap_remove(idx_b);

                let union: Circuit = circ_a.union(&circ_b).cloned().collect();
                circuits.push(union);
            } else if let Some(idx_a) = idx_a {
                let mut circ_a = circuits.swap_remove(idx_a);
                circ_a.insert((*b).clone());
                circuits.push(circ_a);
            } else if let Some(idx_b) = idx_b {
                let mut circ_b = circuits.swap_remove(idx_b);
                circ_b.insert((*a).clone());
                circuits.push(circ_b);
            } else {
                circuits.push(HashSet::from([(*a).clone(), (*b).clone()]));
            }
        });

    circuits
}

static EXAMPLE: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

#[test]
fn test_parse_boxes() {
    let boxes = parse_boxes(EXAMPLE);
    assert_eq!(boxes.len(), 20);
    assert_eq!(boxes[19], [425, 690, 689,]);
}

#[test]
fn test_sorted_distances() {
    let boxes = parse_boxes(EXAMPLE);
    let sorted_distances = sorted_distances(&boxes);
    let closest_boxes = (sorted_distances[0].0, sorted_distances[0].1);
    let second_closest = (sorted_distances[1].0, sorted_distances[1].1);
    assert_eq!(closest_boxes, (&boxes[0], &boxes[19]));
    assert_eq!(second_closest, (&boxes[0], &boxes[7]));
}

#[test]
fn test_connect_closest() {
    let boxes = parse_boxes(EXAMPLE);
    let circuits = connect_closest(&boxes, 3);
    let mut circuit_sizes = circuits.iter().map(HashSet::len).collect::<Vec<usize>>();
    circuit_sizes.sort();
    assert_eq!(circuits.len(), 2);
    assert_eq!(circuit_sizes, vec![2, 3]);
}

#[test]
fn test_solve_1_example() {
    assert_eq!(solve_1(EXAMPLE, 10), 40);
}
