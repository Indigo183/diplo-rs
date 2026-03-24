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
