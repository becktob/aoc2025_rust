use crate::rational::Rational;
use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasherDefault, DefaultHasher};
use std::sync::Mutex;

pub fn solve(part2: bool) -> String {
    let input = std::fs::read_to_string("input_09.txt").expect("could not read file");
    if part2 {
        solve_2(&input).to_string()
    } else {
        solve_1(&input).to_string()
    }
}

fn solve_1(input: &str) -> u64 {
    let floor = parse(input);

    floor
        .iter()
        .enumerate()
        .flat_map(|(i, tile)| floor[i + 1..].iter().map(move |other| (tile, other)))
        .map(|(a, b)| rectangle_size(a, b))
        .max()
        .unwrap()
}

fn solve_2(input: &str) -> u64 {
    let floor = parse(input);
    let contour = contour(&floor);

    let (a, b) = floor
        .iter()
        .enumerate()
        .flat_map(|(i, tile)| floor[i + 1..].iter().map(move |other| (tile, other)))
        .filter(|rect| rect_in_contour(rect, &contour))
        .max_by(|(a, b), (c, d)| rectangle_size(a, b).cmp(&rectangle_size(c, d)))
        .unwrap();

    println!("{:?}", (a, b));
    rectangle_size(a, b)
}

fn rect_in_contour(rect: &Rectangle, contour: &Contour) -> bool {
    let min_x = rect.0.0.min(rect.1.0);
    let max_x = rect.0.0.max(rect.1.0);
    let min_y = rect.0.1.min(rect.1.1);
    let max_y = rect.0.1.max(rect.1.1);
    (min_x..=max_x)
        .flat_map(|x| (min_y..=max_y).map(move |y| (x, y)))
        .all(|tile| tile_in_contour(&tile, contour))
}

type Tile = (i64, i64);
type LineSeg<'a> = (&'a Tile, &'a Tile);
type Rectangle<'a> = (&'a Tile, &'a Tile);
type Floor = Vec<Tile>;
type Contour<'a> = Vec<LineSeg<'a>>;

type Intersection = (Rational, Rational);

fn rectangle_size(&a: &Tile, &b: &Tile) -> u64 {
    (((b.0 - a.0).abs() + 1) * ((b.1 - a.1).abs() + 1)) as u64
}

fn rectangle_inner_contains(rectangle: &Rectangle, tile: &Tile) -> bool {
    let (a, b) = rectangle;
    let xmax = a.0.max(b.0);
    let xmin = a.0.min(b.0);
    let ymax = a.1.max(b.1);
    let ymin = a.1.min(b.1);
    xmin < tile.0 && tile.0 < xmax && ymin < tile.1 && tile.1 < ymax
}

fn tile_in_contour(t: &Tile, contour: &Contour) -> bool {
    let mut vertical_walls: HashMap<i64, Vec<i64>> = HashMap::new();

    contour
        .iter()
        .filter(|line| line.0.0 == line.1.0)
        .for_each(|vertical_line| {
            let x = vertical_line.0.0;
            let y_min = vertical_line.0.1.min(vertical_line.1.1);
            let y_max = vertical_line.0.1.max(vertical_line.1.1);
            (y_min..=y_max).for_each(|y| {
                vertical_walls.entry(y).or_insert(vec![]).push(x);
            })
        });

    vertical_walls.iter_mut().for_each(|(_, v)| v.sort());

    let empty = vec![];
    let crossed_walls: Vec<_> = vertical_walls
        .get(&t.1)
        .unwrap_or(&empty)
        .iter()
        .take_while(|&&wall_y| wall_y <= t.0)
        .collect();

    let tile_is_last_intersection = crossed_walls.contains(&&t.0);

    if tile_is_last_intersection && crossed_walls.len() > 1 {
        crossed_walls.len() % 2 == 0
    } else {
        crossed_walls.len() % 2 == 1
    }
}

fn parse(input: &str) -> Floor {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<_> = line.split(',').map(|s| s.parse().unwrap()).collect();
            (numbers[0], numbers[1])
        })
        .collect()
}

fn contour(floor: &'_ Floor) -> Contour<'_> {
    floor
        .iter()
        .zip(floor[1..].iter().chain(floor[..1].iter()))
        .map(|(a, b)| (a, b))
        .collect()
}

#[cfg(test)]
static EXAMPLE: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

#[test]
fn test_parse() {
    let floor = parse(&EXAMPLE);
    assert_eq!(floor.len(), 8);
    assert_eq!(floor[7], (7, 3));
}

#[test]
fn test_rectangle_size() {
    assert_eq!(rectangle_size(&(0, 0), &(0, 0)), 1);
    assert_eq!(rectangle_size(&(2, 5), &(9, 7)), 24);
    assert_eq!(rectangle_size(&(7, 1), &(11, 7)), 35);
    assert_eq!(rectangle_size(&(2, 5), &(11, 1)), 50);
}

#[test]
fn test_rectangle_inner_contains() {
    let largest_example_rectangle = (&(2, 5), &(11, 1));
    assert!(rectangle_inner_contains(
        &largest_example_rectangle,
        &(7, 3)
    ))
}

#[test]
fn test_tile_in_countour() {
    let floor = parse(EXAMPLE);
    let contour: Contour = contour(&floor);
    let tile_truly_inside = (3, 4);
    let first_corner = floor[0];
    let middle_corner = floor[4];
    let tile_beyond_contour = (12, 12);
    assert!(tile_in_contour(&tile_truly_inside, &contour));
    assert!(tile_in_contour(&first_corner, &contour)); // breaks when contour is closed
    assert!(tile_in_contour(&middle_corner, &contour));
    assert!(!tile_in_contour(&tile_beyond_contour, &contour));

    floor.iter().for_each(|tile| {
        assert!(tile_in_contour(&tile, &contour));
    })
}

#[test]
fn test_solve_1_example() {
    assert_eq!(solve_1(&EXAMPLE), 50);
}

#[test]
fn test_solve_1() {
    assert_eq!(solve(false), "4759930955")
}

#[test]
fn test_solve_2_example() {
    assert_eq!(solve_2(&EXAMPLE), 24);
}

#[ignore]
#[test]
fn test_solve_2() {
    assert_eq!(solve(true), "42");
    // 4621384368 too high
}
