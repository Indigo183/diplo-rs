use std::collections::HashMap;

use diplo_rs::province::*;

fn main() {
    println!("Hello, world!")
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct LocalProvince {
    /// The full name of the province.
    name: &'static str,
    /// Whether the province is a supply centre.
    is_centre: bool,
    /// Whether an army can move to the province.
    is_land: bool,
    /// Whether a fleet can convoy from the province.
    is_water: bool,
    /// A list containing all coasts of the province.
    coasts: Vec<Coast>,
    /// A HashMap that returns whether the province is adjacent for Armies and Fleets, respectively.
    adjacencies: HashMap<&'static str, (bool, bool)>,
}

impl LocalProvince {
    fn new(
        name: &'static str,
        is_centre: bool,
        is_land: bool,
        is_water: bool,
        coasts: Vec<Coast>,
        input_adjacencies: Vec<(&'static str, bool, bool)>,
    ) -> LocalProvince {
        let mut adjacencies = HashMap::with_capacity(input_adjacencies.len());
        for (name, army, fleet) in input_adjacencies {
            adjacencies.insert(name, (army, fleet));
        }
        LocalProvince {
            name,
            is_centre,
            is_land,
            is_water,
            coasts,
            adjacencies,
        }
    }
}

impl Province for LocalProvince {
    fn is_centre(&self) -> bool {
        self.is_centre
    }
    fn is_land(&self) -> bool {
        self.is_land
    }
    fn is_water(&self) -> bool {
        self.is_water
    }
    fn get_coasts(&self) -> &Vec<Coast> {
        &self.coasts
    }

    fn is_adjacent_to(&self, other: &Self, unit: Unit) -> bool {
        let &(army, fleet) = self.adjacencies.get(other.name).unwrap_or(&(false, false));
        match unit {
            Unit::Army => army,
            Unit::Fleet => fleet,
        }
    }
}
