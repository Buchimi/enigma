pub struct Reflector {
    mapping: Vec<(&'static str, &'static str)>,
}

impl Reflector {
    pub fn reflector_b() -> Reflector {
        let map = vec![
            ("A", "Y"),
            ("B", "R"),
            ("C", "U"),
            ("D", "H"),
            ("E", "Q"),
            ("F", "S"),
            ("G", "L"),
            ("I", "P"),
            ("J", "X"),
            ("K", "N"),
            ("M", "O"),
            ("T", "Z"),
            ("V", "W"),
        ];
        //TODO fill in the structs

        Reflector { mapping: map }
    }
    pub fn reflect(&self, character: &str) -> &'static str {
        for (char1, char2) in self.mapping.iter() {
            if (character.eq(*char1)) {
                return *char2;
            } else if (character.eq(*char2)) {
                return *char1;
            }
        }
        panic!("aye, something is not being reflected")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reflection() {
        let reflector = Reflector::reflector_b();
        for i in 0..26 {
            reflector.reflect(((i + 65) as u8 as char).to_string().as_str());
        }
    }
}
