use std::collections::HashMap;
use std::iter;

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
    let wall_directions = vertical_wall_directions(&floor);

    let mut rectangles: Vec<_> = floor
        .iter()
        .enumerate()
        .flat_map(|(i, tile)| floor[i + 1..].iter().map(move |other| (tile, other)))
        .collect();

    rectangles.sort_by(|(a, b), (c, d)| rectangle_size(a, b).cmp(&rectangle_size(c, d)));
    rectangles.reverse();

    let (a, b) = rectangles
        .iter()
        .filter(|rect| rect_in_contour(rect, &wall_directions))
        .next()
        .unwrap();

    // println!("{:?}", (a, b));
    rectangle_size(a, b)
}

fn rect_in_contour(rect: &Rectangle, wall_directions: &WallDirections) -> bool {
    let min_x = rect.0.0.min(rect.1.0);
    let max_x = rect.0.0.max(rect.1.0);
    let min_y = rect.0.1.min(rect.1.1);
    let max_y = rect.0.1.max(rect.1.1);

    let tile_not_in_contour = (min_y..=max_y)
        .filter_map(|y| {
            let Some(walls_this_line) = wall_directions.get(&y) else { return None };
            let walls_in_rect = walls_this_line
                .iter()
                .filter(|(wall_x, _)| min_x < *wall_x && *wall_x < max_x);

            let wall_left_of_rect = walls_this_line
                .iter()
                .take_while(|(wall_x, _)| *wall_x <= min_x)
                .last()
                .unwrap_or(&(0, true));

            let not_in_contour = walls_in_rect
                .chain(iter::once(wall_left_of_rect))
                .filter(|(_, up)| *up)
                .next();
            not_in_contour.map(|(x, _)| (x, y))
        })
        .next();

    tile_not_in_contour.is_none()
}

type Tile = (i64, i64);
type LineSeg<'a> = (&'a Tile, &'a Tile);
type Rectangle<'a> = (&'a Tile, &'a Tile);
type Floor = Vec<Tile>;
type Contour<'a> = Vec<LineSeg<'a>>;

fn rectangle_size(&a: &Tile, &b: &Tile) -> u64 {
    (((b.0 - a.0).abs() + 1) * ((b.1 - a.1).abs() + 1)) as u64
}

type WallDirections = HashMap<i64, Vec<(i64, bool)>>;
fn vertical_wall_directions(floor: &Floor) -> WallDirections {
    let mut vertical_walls = HashMap::new();

    contour(floor)
        .iter()
        .filter(|line| line.0.0 == line.1.0)
        .for_each(|vertical_line| {
            let x = vertical_line.0.0;
            let y_min = vertical_line.0.1.min(vertical_line.1.1);
            let y_max = vertical_line.0.1.max(vertical_line.1.1);
            let upwards = vertical_line.0.1 < vertical_line.1.1;
            (y_min..=y_max).for_each(|y| {
                vertical_walls.entry(y).or_insert(vec![]).push((x, upwards));
            })
        });

    vertical_walls.iter_mut().for_each(|(_, v)| v.sort());

    vertical_walls
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
fn test_rect_in_contour() {
    let floor = parse(EXAMPLE);
    let wall_directions = vertical_wall_directions(&floor);
    let example_rectangle = (&(9, 5), &(2, 3)); // example solution to part 2
    assert!(rect_in_contour(&example_rectangle, &wall_directions));
}

#[test]
fn test_rect_in_contour_97_23() {
    // (2, 6) is not in contour
    let floor = parse(EXAMPLE);
    let wall_directions = vertical_wall_directions(&floor);
    let example_rectangle = (&(9, 7), &(2, 3)); // example solution to part 2
    assert!(!rect_in_contour(&example_rectangle, &wall_directions));
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

//#[ignore]
#[test]
fn test_solve_2() {
    assert_eq!(solve(true), "1525241870");
}
