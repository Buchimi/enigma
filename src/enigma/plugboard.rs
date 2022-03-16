pub struct PlugBoard {
    mapping: Vec<(&'static str, &'static str)>,
}

impl PlugBoard {
    //sample plugboard
    pub fn new() -> PlugBoard {
        let map = vec![
            ("A", "D"),
            ("C", "N"),
            ("E", "T"),
            ("F", "L"),
            ("G", "I"),
            ("J", "V"),
            ("K", "Z"),
            ("P", "U"),
            ("Q", "Y"),
            ("W", "X"),
        ];
        //TODO fill in the structs

        PlugBoard { mapping: map }
    }

    pub fn swap(&self, character: &'static str) -> &'static str {
        for (char1, char2) in self.mapping.iter() {
            if (character.eq(*char1)) {
                return *char2;
            } else if (character.eq(*char2)) {
                return *char1;
            }
        }
        return character;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_swapping() {
        let board = PlugBoard::new();
        assert_eq!(board.swap("A"), "D");
        assert_eq!(board.swap("N"), "C");
        assert_eq!(board.swap("B"), "B");
    }
}
