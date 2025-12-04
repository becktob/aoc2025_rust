use std::collections::BTreeMap;

mod p01;
mod p02;
mod p03;
mod p04;

fn main() {
    type SolveFunc = fn(bool) -> String;
    let modules = BTreeMap::from([
        ("01", p01::solve as SolveFunc),
        ("02", p02::solve as SolveFunc),
        ("03", p03::solve as SolveFunc),
        ("04", p04::solve as SolveFunc),
    ]);

    for (name, func) in modules {
        let solution1 = func(false);
        let solution2 = func(true);
        println!("{name} part 1: [{solution1}], part 2: [{solution2}]");
    }
}
