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

#[derive(Debug, Eq, PartialEq)]
struct Box {
    x: u64,
    y: u64,
    z: u64,
}

fn parse_boxes(input: &str) -> Vec<Box> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .map(|xyz: Vec<u64>| Box {
            x: xyz[0],
            y: xyz[1],
            z: xyz[2],
        })
        .collect()
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
    assert_eq!(
        boxes[19],
        Box {
            x: 425,
            y: 690,
            z: 689
        }
    );
}
