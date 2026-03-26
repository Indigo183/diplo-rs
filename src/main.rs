use std::collections::HashMap;

use diplo_rs::default;
use diplo_rs::province::*;

fn main() {
    let [cat, cae, bru, pom] = default::DefaultProvince::romans();

    for unit in [Unit::Army, Unit::Fleet] {
        for x in [&cat, &cae, &bru, &pom] {
            for y in [&cat, &cae, &bru, &pom] {
                if x.is_adjacent_to(&y, unit) {
                    println!("{:?} in {} is adjacent to {}", unit, x.id(), y.id());
                }
            }
        }
    }
}
