use super::*;

impl DefaultProvince {
    pub fn romans() -> [DefaultProvince; 4] {
        [
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
        ]
    }
}

#[cfg(test)]
mod test_roman {
    use super::*;

    #[test]
    fn adjacencies() {
        let romans = DefaultProvince::romans();
        let mut adjacencies = Vec::with_capacity(16);

        for unit in [Unit::Army, Unit::Fleet] {
            for x in &romans {
                for y in &romans {
                    let adjacent = x.is_adjacent_to(&y, unit);
                    adjacencies.push(adjacent);
                    if adjacent {
                        println!("{:?} in {} is adjacent to {}", unit, x.id(), y.id());
                    }
                }
            }
        }

        assert_eq!(
            adjacencies,
            [
                false, true, true, false, true, false, true, true, true, false, true, true, false,
                true, true, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false,
            ]
        )
    }
}
