use crate::p12::{
    EXAMPLE, PresentShape, RegionMap, empty_region, parse, print_region_map, put_shape_into,
};
use std::iter;

pub fn solve(part2: bool) -> String {
    let _input = std::fs::read_to_string("input_12.txt").expect("could not read file");
    if part2 {
        "WIP".to_string()
        //solve_2(&input).to_string()
    } else {
        "WIP".to_string()
        //crate::p10::solve_1(&input).to_string()
    }
}

// Idea: Do all pairwise colliding once (including scanning, rotating...)
// -> Search in large region is simpler and hashable

#[derive(Debug, PartialEq)]
struct PairOrientation {
    // delta = [a to b] = i_b - i_a
    delta_i: i32,
    delta_j: i32,
    rot_a: i8,
    rot_b: i8,
}

fn pair_collides(a: &PresentShape, b: &PresentShape, o: &PairOrientation) -> bool {
    let region_with_both = combine_pair(a, b, o);
    region_with_both.is_none()
}

fn combine_pair(a: &PresentShape, b: &PresentShape, o: &PairOrientation) -> Option<RegionMap> {
    // todo: less cloning
    let (i_a, j_a) = (4, 4);
    let (i_b, j_b) = (i_a + o.delta_i, j_a + o.delta_j);

    let empty_region = empty_region(9, 9);
    let region_with_a = put_shape_into(
        &empty_region,
        &a,
        (i_a as usize, j_a as usize),
        o.rot_a as usize,
    )
    .unwrap(); // todo: lots of cloning in here

    put_shape_into(
        &region_with_a,
        &b,
        (i_b as usize, j_b as usize),
        o.rot_b as usize,
    )
}

fn collision_map(a: &PresentShape, b: &PresentShape) -> Vec<PairOrientation> {
    // Todo: make use of symmetries (e.g. delta > 0)
    // Todo: all_orientations could be computed one single time
    // Todo: store non-collisions instead (inside 3x3, there are less non-c than collisions)
    let all_orientations = (-3..=3)
        .flat_map(|delta_i| {
            (-3..=3).flat_map(move |delta_j| {
                (0..4).flat_map(move |rot_a| {
                    (0..4).map(move |rot_b| PairOrientation {
                        delta_i,
                        delta_j,
                        rot_a,
                        rot_b,
                    })
                })
            })
        })
        .collect::<Vec<PairOrientation>>();
    all_orientations
        .into_iter()
        .filter(|o| pair_collides(a, b, o))
        .collect()
}

#[test]
fn test_collision_map() {
    let (presents, regions) = parse(EXAMPLE);
    let collisions = collision_map(&presents[4], &presents[4]);

    let non_collisions = massive_block_collisions()
        .into_iter()
        .filter(|c| !collisions.contains(c))
        .collect::<Vec<_>>();
    non_collisions.iter().for_each(|p| println!("{:?}", p));

    // all combinations of two interlocking 'C's
    // 4 rot_a
    // *2 C-openings offset left/right
    // *2 interlock by 1 or by 2

    assert_eq!(non_collisions.len(), 4 * 2 * 2);
}

#[test]
fn test_collision_map_massive_block() {
    let collisions = massive_block_collisions();
    assert_eq!(collisions.len(), 25 * 4 * 4);
}

#[cfg(test)]
fn massive_block_collisions() -> Vec<PairOrientation> {
    let block = iter::repeat_n(iter::repeat_n(true, 3).collect(), 3).collect();
    let collisions = collision_map(&block, &block);
    collisions
}

#[test]
fn test_collides_exploratory() {
    let (presents, _) = parse(EXAMPLE);
    let o = PairOrientation {
        delta_i: -2,
        delta_j: -2,
        rot_a: 2,
        rot_b: 0,
    };

    assert!(pair_collides(&presents[4], &presents[4], &o));
}
