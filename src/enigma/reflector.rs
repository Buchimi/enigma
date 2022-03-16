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
}