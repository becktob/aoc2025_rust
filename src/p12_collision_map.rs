use crate::p12_grid::{EXAMPLE, PresentShape, Region, RegionMap, empty_region, parse, put_shape_into};
use std::collections::{HashMap, HashSet};
use std::iter;

pub fn solve(part2: bool) -> String {
    let input = std::fs::read_to_string("input_12.txt").expect("could not read file");
    if part2 {
        "WIP".to_string()
        //solve_2(&input).to_string()
    } else {
        solve_1(&input).to_string()
    }
}

// Idea: Do all pairwise colliding once (including scanning, rotating...)
// -> Search in large region is simpler and hashable

fn solve_1(input: &str) -> usize {
    let (presents, regions) = parse(input);
    regions
        .iter()
        .filter_map(|region| fill_region(region, &presents))
        .count()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct PairOrientation {
    // delta = [a to b] = i_b - i_a
    delta_i: i32,
    delta_j: i32,
    rot_a: u8,
    rot_b: u8,
}

type CompatibilityMap = HashMap<(u8, u8), HashSet<PairOrientation>>;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct PlacedPresent {
    n: u8,
    i: i32,
    j: i32,
    rot: u8,
}

fn pair_collides(a: &PresentShape, b: &PresentShape, o: &PairOrientation) -> bool {
    let region_with_both = combine_pair(a, b, o);
    region_with_both.is_none()
}

fn combine_pair(a: &PresentShape, b: &PresentShape, o: &PairOrientation) -> Option<RegionMap> {
    // todo: less cloning
    let (i_a, j_a) = (2usize, 2usize);
    let (i_b, j_b) = (
        i_a.checked_add_signed(o.delta_i as isize).unwrap(),
        j_a.checked_add_signed(o.delta_j as isize).unwrap(),
    );

    let empty_region = empty_region(5, 5);
    let region_with_a = put_shape_into(
        &empty_region,
        &a,
        (i_a, j_a),
        o.rot_a as usize,
    )
    .unwrap(); // todo: lots of cloning in here

    put_shape_into(
        &region_with_a,
        &b,
        (i_b, j_b),
        o.rot_b as usize,
    )
}

fn compatibility_pair(a: &PresentShape, b: &PresentShape) -> Vec<PairOrientation> {
    // Todo: make use of symmetries (e.g. delta > 0)
    // Todo: all_orientations could be computed one single time
    let all_orientations = (-2..=2)
        .flat_map(|delta_i| {
            (-2..=2).flat_map(move |delta_j| {
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
        .filter(|o| !pair_collides(a, b, o))
        .collect()
}

fn compatibility_map(presents: &Vec<PresentShape>) -> CompatibilityMap {
    presents
        .iter()
        .enumerate()
        .flat_map(|(m, a)| {
            presents.iter().enumerate().map(move |(n, b)| {
                let compatibility = compatibility_pair(a, b)
                    .into_iter()
                    .collect::<HashSet<PairOrientation>>();
                ((m as u8, n as u8), compatibility)
            })
        })
        .collect()
}

fn fill_region(region: &Region, shapes: &Vec<PresentShape>) -> Option<HashSet<PlacedPresent>> {
    let compatibility = compatibility_map(shapes);
    let shapes_todo = region
        .presets_needed
        .iter()
        .zip(0..)
        .flat_map(|(&times_needed, shape_no)| iter::repeat_n(shape_no, times_needed))
        .collect();
    let w_h = (region.width, region.height);
    fill_iter(shapes_todo, HashSet::new(), w_h, &compatibility)
}

fn fill_iter(
    presents_todo: Vec<u8>,
    present_positions: HashSet<PlacedPresent>,
    w_h: (usize, usize),
    compatibility: &CompatibilityMap,
) -> Option<HashSet<PlacedPresent>> {
    if presents_todo.is_empty() {
        return Some(present_positions);
    }

    let this_present = presents_todo[0];
    let remaining_presents = presents_todo[1..].to_vec();

    let (w, h) = w_h;
    let present_size = 3; // todo hardcoded
    let w_max = w as i32 - present_size;
    let h_max = h as i32 - present_size;

    let h_min = present_positions
        .iter()
        .filter(|p| p.n == this_present)
        .min_by_key(|&p| p.i)
        .map_or(0, |p| p.i);
    let w_min = present_positions
        .iter()
        .filter(|p| p.n == this_present)
        .min_by_key(|&p| p.j)
        .map_or(0, |p| p.j);

    let all_orientations = (h_min..=h_max)
        .flat_map(|i| {
            (w_min..=w_max).flat_map(move |j| {
                (0..4).map(move |rot| PlacedPresent {
                    n: this_present,
                    i,
                    j,
                    rot,
                })
            })
        })
        .collect::<Vec<PlacedPresent>>();

    let fits = |o: &PlacedPresent| -> bool {
        present_positions.iter().all(|p| {
            let delta_i = p.i - o.i;
            let delta_j = p.j - o.j;
            if delta_i.abs() >= present_size || delta_j.abs() >= present_size {
                return true;
            }

            let pair = PairOrientation {
                delta_i,
                delta_j,
                rot_a: o.rot,
                rot_b: p.rot,
            };
            compatibility.get(&(o.n, p.n)).unwrap().contains(&pair)
        })
    };

    let fillings = all_orientations
        .into_iter()
        .filter(fits)
        .filter_map(|new_position| {
            let positions = present_positions
                .iter()
                .cloned()
                .chain(iter::once(new_position))
                .collect();
            fill_iter(remaining_presents.clone(), positions, w_h, compatibility)
        })
        .next();

    if fillings.is_none() { None } else { fillings }
}

#[test]
fn test_collision_pair() {
    let (presents, regions) = parse(EXAMPLE);
    let non_collisions = compatibility_pair(&presents[4], &presents[4]);

    // all combinations of two interlocking 'C's
    // 4 rot_a
    // *2 C-openings offset left/right
    // *2 interlock by 1 or by 2

    assert_eq!(non_collisions.len(), 4 * 2 * 2);
}

#[test]
fn test_collision_pair_massive_block() {
    let block = iter::repeat_n(iter::repeat_n(true, 3).collect(), 3).collect();
    let compatibility = compatibility_pair(&block, &block);
    assert_eq!(compatibility.len(), 0);
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

#[test]
fn test_collision_map() {
    let (presents, _) = parse(EXAMPLE);
    let map = compatibility_map(&presents);
    assert_eq!(map.len(), presents.len() * presents.len());

    let collisions_4_4 = map.get(&(4, 4)).unwrap();
    assert_eq!(collisions_4_4.len(), 16); // see test_collision_pair()
}

#[test]
fn test_fill_iter_puts_one_present() {
    let (presents, _) = parse(EXAMPLE);
    let collisions = compatibility_map(&presents);
    let presents_todo = vec![4];
    let filling = fill_iter(presents_todo, HashSet::new(), (3, 3), &collisions);

    assert!(filling.is_some());
}

#[test]
fn test_fill_iter_cant_put_two_presents() {
    let (presents, _) = parse(EXAMPLE);
    let collisions = compatibility_map(&presents);
    let presents_todo = vec![4, 4];
    let filling = fill_iter(presents_todo, HashSet::new(), (3, 3), &collisions);

    assert!(filling.is_none());
}

#[test]
fn test_fill_iter_can_put_two_presents_in_4_by_4() {
    let (presents, _) = parse(EXAMPLE);
    let collisions = compatibility_map(&presents);
    let presents_todo = vec![4, 4];
    let fillings = fill_iter(presents_todo, HashSet::new(), (4, 4), &collisions);

    assert!(fillings.is_some());
}

#[test]
fn test_fill_region() {
    let (presents, regions) = parse(EXAMPLE);
    assert!(fill_region(&regions[0], &presents).is_some());
}

#[test]
fn test_fill_region_no_solution() {
    let (presents, _) = parse(EXAMPLE);
    let region = Region {
        width: 4,
        height: 12, // 6 fit into 12x4
        presets_needed: vec![0, 0, 0, 0, 7, 0],
    };
    let filling = fill_region(&region, &presents);
    assert!(filling.is_none());
    // 34s -> 4s21
}

#[test]
fn test_fill_region_2() {
    let (presents, regions) = parse(EXAMPLE);
    let filling = fill_region(&regions[1], &presents);
    assert!(filling.is_some());
}

#[test]
fn solve_1_example() {
    assert_eq!(solve_1(EXAMPLE), 2)
    // 5min23 -> 1min2
}
