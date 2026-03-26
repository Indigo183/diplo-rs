pub mod province {

    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Unit {
        Army,
        Fleet,
    }

    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum Coast {
        NC,
        SC,
        EC,
        WC,
    }

    pub trait Province {
        // DATA //

        /// The province's identifier.
        fn id(&self) -> &str;
        /// Whether the province is a supply centre.
        fn is_centre(&self) -> bool;
        /// Whether an army can move to the province.
        fn is_land(&self) -> bool;
        /// Whether a fleet can convoy from the province.
        fn is_water(&self) -> bool;

        // METHODS //

        /// Checks if the provinces are adjacent for a given unit type.
        fn is_adjacent_to(&self, other: &Self, unit: Unit) -> bool;
        /// Returns a list containing all coasts of the province.
        fn get_coasts(&self) -> &Vec<Coast>;
        /// Whether the province has multiple coasts.
        fn has_coasts(&self) -> bool {
            !self.get_coasts().is_empty()
        }
    }
}

pub mod default {
    use std::collections::HashMap;

    use crate::province::*;

    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct DefaultProvince {
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

    impl DefaultProvince {
        pub fn new(
            name: &'static str,
            is_centre: bool,
            is_land: bool,
            is_water: bool,
            coasts: Vec<Coast>,
            input_adjacencies: Vec<(&'static str, bool, bool)>,
        ) -> Self {
            let mut adjacencies = HashMap::with_capacity(input_adjacencies.len());
            for (name, army, fleet) in input_adjacencies {
                adjacencies.insert(name, (army, fleet));
            }
            DefaultProvince {
                name,
                is_centre,
                is_land,
                is_water,
                coasts,
                adjacencies,
            }
        }

        pub fn romans() -> (
            DefaultProvince,
            DefaultProvince,
            DefaultProvince,
            DefaultProvince,
        ) {
            (
                DefaultProvince::new(
                    "Cato",
                    true,
                    true,
                    false,
                    Vec::new(),
                    vec![("Caesar", true, false), ("Brutus", true, false)],
                ),
                DefaultProvince::new(
                    "Caesar",
                    false,
                    true,
                    false,
                    Vec::new(),
                    vec![
                        ("Cato", true, false),
                        ("Brutus", true, false),
                        ("Pompey", true, false),
                    ],
                ),
                DefaultProvince::new(
                    "Brutus",
                    false,
                    true,
                    false,
                    Vec::new(),
                    vec![
                        ("Cato", true, false),
                        ("Brutus", true, false),
                        ("Pompey", true, false),
                    ],
                ),
                DefaultProvince::new(
                    "Pompey",
                    true,
                    true,
                    false,
                    Vec::new(),
                    vec![("Caesar", true, false), ("Brutus", true, false)],
                ),
            )
        }
    }

    impl Province for DefaultProvince {
        fn id(&self) -> &str {
            self.name
        }
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
}
