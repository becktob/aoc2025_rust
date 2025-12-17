use crate::helpers;
use std::iter;

pub fn solve(part2: bool) -> String {
    let _input = std::fs::read_to_string("input_10.txt").expect("could not read file");
    if part2 {
        "WIP".to_string()
        //solve_2(&input).to_string()
    } else {
        "WIP".to_string()
        //crate::p10::solve_1(&input).to_string()
    }
}

type PresentShape = Vec<Vec<bool>>;
type RegionMap = Vec<Vec<bool>>;
struct Region {
    width: usize,
    height: usize,
    presets_needed: Vec<usize>,
}

fn empty_region(w: usize, h: usize) -> RegionMap {
    iter::repeat_n(iter::repeat_n(false, w).collect(), h).collect()
}

fn parse(input: &str) -> (Vec<PresentShape>, Vec<Region>) {
    let (presents_raw, regions_raw) = input.rsplit_once("\n\n").unwrap();

    let presents = presents_raw.split("\n\n").map(parse_shape).collect();
    let regions = regions_raw.lines().map(parse_region).collect();

    (presents, regions)
}

fn parse_shape(input: &str) -> PresentShape {
    input
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn parse_region(input: &str) -> Region {
    let (wh_raw, presents_raw) = input.split_once(':').unwrap();
    let wh = wh_raw
        .split('x')
        .map(str::parse)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    let (width, height) = (wh[0], wh[1]);
    let presets_needed = presents_raw
        .trim()
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    Region {
        width,
        height,
        presets_needed,
    }
}

fn put_shape_into(
    region_map: &RegionMap,
    shape: &PresentShape,
    offset: (usize, usize),
    rot90: usize,
) -> Option<RegionMap> {
    // todo: less cloning, more refs/views
    let mut shape = shape.clone();
    (0..rot90).for_each(|_| shape = helpers::rot90(shape.clone()));

    let fits = !region_map[offset.0..]
        .iter()
        .zip(shape.iter())
        .map(|(region_row, shape_row)| {
            region_row[offset.1..]
                .iter()
                .zip(shape_row.iter())
                .any(|(&r, &s)| r && s)
        })
        .any(|both| both);

    if fits {
        // insert_piece
        let mut filled_map = region_map.clone();
        filled_map //[offset.0..]
            .iter_mut()
            .zip(shape.iter())
            .for_each(|(region_row, shape_row)| {
                region_row[offset.1..]
                    .iter_mut()
                    .zip(shape_row.iter())
                    .for_each(|(r, &s)| *r = s)
            });
        Some(filled_map)
    } else {
        None
    }
}

fn fill_region(region: &Region, shapes: &Vec<PresentShape>) -> Option<RegionMap> {
    let shapes_todo: Vec<_> = shapes
        .iter()
        .zip(region.presets_needed.iter())
        .flat_map(|(shape, &times_needed)| iter::repeat_n(shape, times_needed))
        .cloned()
        .collect();

    let region_in_progess = empty_region(region.width, region.height);

    fill_region_iter(region_in_progess, shapes_todo)
        .iter()
        .next()?
        .get(0)
        .cloned()
}

fn fill_region_iter(
    region_in_progress: RegionMap,
    shapes_todo: Vec<PresentShape>,
) -> Option<Vec<RegionMap>> {
    if shapes_todo.is_empty() {
        // nothing to do, this is a solution!
        return Some(vec![region_in_progress]);
    }

    let this_shape = shapes_todo[0].clone();

    let present_size = 3;
    let max_offset_h = region_in_progress.len() - present_size;
    let max_offset_w = region_in_progress[0].len() - present_size;

    let try_to_insert_this_shape = |n_rot, i, j| {
        let region_clone = region_in_progress.iter().cloned().collect();
        let region_with_this_shape = put_shape_into(&region_clone, &this_shape, (i, j), n_rot);
        region_with_this_shape
    };

    let fill_region_with_remaining = |region_with_this_shape: RegionMap| {
        let remaining_shapes = shapes_todo[1..].iter().cloned().collect::<Vec<_>>();
        fill_region_iter(region_with_this_shape.clone(), remaining_shapes)
    };

    let fillings = (0..4)
        .flat_map(|n_rot| {
            (0..=max_offset_h).flat_map(move |i| {
                (0..=max_offset_w)
                    .filter_map(move |j| try_to_insert_this_shape(n_rot, i, j))
                    .filter_map(move |region_with_this_shape| {
                        fill_region_with_remaining(region_with_this_shape)
                    })
                    .flatten()
            })
        })
        .collect::<Vec<_>>();

    if fillings.is_empty() {
        None
    } else {
        Some(fillings)
    }
}

static EXAMPLE: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

#[test]
fn test_parse() {
    let (presents, regions) = parse(EXAMPLE);
    assert_eq!(presents.len(), 6);
    assert_eq!(regions.len(), 3);
}

#[test]
fn test_put_shape_into_empty() {
    let (presents, _) = parse(EXAMPLE);
    let empty_region = empty_region(4, 4);
    let region_with_piece = put_shape_into(&empty_region, &presents[4], (0, 0), 0);
    assert!(region_with_piece.is_some());
}

#[test]
fn test_put_shape_into_not_twice() {
    let (presents, _) = parse(EXAMPLE);
    let empty_region = empty_region(4, 4);
    assert_eq!(empty_region[0][0], false);
    let region_with_first = put_shape_into(&empty_region, &presents[4], (0, 0), 0);
    assert!(region_with_first.is_some());
    let region_with_first = region_with_first.unwrap();
    assert_eq!(region_with_first[0][0], true);
    let region_with_second = put_shape_into(&region_with_first, &presents[4], (0, 0), 0);
    assert!(region_with_second.is_none());
}

#[test]
fn test_put_shape_into_rotated() {
    let (presents, _) = parse(EXAMPLE);
    let empty_region = empty_region(4, 4);
    let region_with_first = put_shape_into(&empty_region, &presents[4], (0, 0), 0);
    assert!(region_with_first.is_some());
    let region_with_rotated = put_shape_into(&empty_region, &presents[4], (1, 1), 2);
    assert!(region_with_rotated.is_some());
}

#[test]
fn test_fill_region() {
    let (presents, regions) = parse(EXAMPLE);
    let filling = fill_region(&regions[0], &presents);
    assert!(filling.is_some());

    let filled_count = filling.unwrap().iter().flatten().filter(|&p| *p).count();
    assert_eq!(filled_count, 2 * 7);
}

#[test]
fn test_fill_region_iter_terminates_when_nothing_left_todo() {
    let region = empty_region(2, 2);
    let todo = vec![];
    let iter_result = fill_region_iter(region, todo);
    assert!(iter_result.is_some());
}

#[test]
fn test_fill_region_iter_inserts_single_piece() {
    let (presents, _) = parse(EXAMPLE);
    let region = empty_region(3, 3);
    let p4 = presents[4].iter().cloned().collect::<Vec<_>>();
    let todo = vec![p4.clone()];
    let iter_result = fill_region_iter(region, todo);
    assert!(iter_result.is_some());
    let fillings = iter_result.unwrap();
    assert_eq!(fillings.len(), 4);  // 4 rotations
    let filled_count = fillings[0].iter().flatten().filter(|&p| *p).count();
    let present_count = p4.iter().flatten().filter(|&p| *p).count();
    assert_eq!(filled_count, present_count);
}
