pub fn solve(part2: bool) -> String {
    let input = std::fs::read_to_string("input_12.txt").expect("could not read file");
    if part2 {
        "DONE - solved everything else 🌈".to_string()
    } else {
        solve_1(&input).to_string()
    }
}

fn solve_1(input: &String) -> usize {
    let (presents, regions) = parse(&input);
    let possible = trivially_possible(&regions);
    let impossible = trivially_impossible(&presents, &regions);
    assert_eq!(possible+impossible, regions.len());
    possible
}

#[test]
fn test_trivially_possible() {
    let input = std::fs::read_to_string("input_12.txt").expect("could not read file");
    let (_, regions) = parse(&input);
    let trivial = trivially_possible(&regions);
    assert_eq!(regions.len(), 1000);
    assert_eq!(trivial, 526)
}

#[test]
fn test_trivially_impossible() {
    let input = std::fs::read_to_string("input_12.txt").expect("could not read file");
    let (presents, regions) = parse(&input);
    let impossible = trivially_impossible(&presents, &regions);
    assert_eq!(regions.len(), 1000);
    assert_eq!(impossible, 474)
}

pub fn trivially_possible(regions: &Vec<Region>) -> usize {
    let trivial = regions.iter().filter(could_fill_blockwise).count();
    trivial
}

fn could_fill_blockwise(region: &&Region) -> bool {
    let present_rows = region.height.div_euclid(3);
    let present_cols = region.width.div_euclid(3);
    let trivially_fittable = present_cols * present_rows;
    let total_presents = region.presets_needed.iter().sum::<usize>();
    total_presents <= trivially_fittable
}

pub fn trivially_impossible(presents: &Vec<PresentShape>, regions: &Vec<Region>) -> usize {
    regions
        .iter()
        .filter(|r| cant_even_fit_tiles(r, &presents))
        .count()
}

fn cant_even_fit_tiles(region: &&Region, presents: &Vec<PresentShape>) -> bool {
    let present_sizes = presents
        .iter()
        .map(|p| {
            p.iter()
                .map(|r| r.iter().filter(|&&p| p).count())
                .sum::<usize>()
        })
        .collect::<Vec<_>>();

    let region_tiles = region.width * region.height;
    let present_tiles = region
        .presets_needed
        .iter()
        .zip(present_sizes.iter())
        .map(|(num, size)| num * size)
        .sum::<usize>();
    present_tiles > region_tiles
}

pub(crate) type PresentShape = Vec<Vec<bool>>;

pub(crate) struct Region {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) presets_needed: Vec<usize>,
}

pub(crate) fn parse(input: &str) -> (Vec<PresentShape>, Vec<Region>) {
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

#[cfg(test)]
pub(crate) static EXAMPLE: &str = "0:
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