use crate::p12_grid::{Region, parse};

#[test]
fn trivially_possible() {
    let input = std::fs::read_to_string("input_12.txt").expect("could not read file");
    let (_, regions) = parse(&input);
    let trivial = regions.iter().filter(could_fill_blockwise).count();
    assert_eq!(regions.len(), 1000);
    assert_eq!(trivial, 526)
}

fn could_fill_blockwise(region: &&Region) -> bool {
    let present_rows = region.height.div_euclid(3);
    let present_cols = region.width.div_euclid(3);
    let trivially_fittable = present_cols * present_rows;
    let total_presents = region.presets_needed.iter().sum::<usize>();
    total_presents <= trivially_fittable
}
