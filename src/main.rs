use std::collections::BTreeMap;

mod p01;
mod p02;
mod p03;
mod p04;
mod p05;
mod p06;
mod p07;
mod p08;
mod p09;
mod p10;
mod p11;
mod p12;
mod helpers;
mod p12_collision_map;

fn main() {
    type SolveFunc = fn(bool) -> String;
    let modules = BTreeMap::from([
        ("01", p01::solve as SolveFunc),
        ("02", p02::solve as SolveFunc),
        ("03", p03::solve as SolveFunc),
        ("04", p04::solve as SolveFunc),
        ("05", p05::solve as SolveFunc),
        ("06", p06::solve as SolveFunc),
        ("07", p07::solve as SolveFunc),
        ("08", p08::solve as SolveFunc),
        ("09", p09::solve as SolveFunc),
        ("10", p10::solve as SolveFunc),
        ("11", p11::solve as SolveFunc),
        ("12", p12::solve as SolveFunc),
    ]);

    for (name, func) in modules {
        let solution1 = func(false);
        let solution2 = func(true);
        println!("{name} part 1: [{solution1}], part 2: [{solution2}]");
    }
}
