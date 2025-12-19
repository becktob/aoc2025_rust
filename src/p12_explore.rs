use crate::p12_grid::{PresentShape, Region, parse};

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
