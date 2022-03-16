use std::collections::HashMap;
pub struct PlugBoard {
    mapping: HashMap<&'static str, &'static str>,
}

impl PlugBoard {
    fn new() -> PlugBoard {
        let mut map: HashMap<&'static str, &'static str> = HashMap::new();
        map.insert("A", "B");
        //TODO fill in the structs

        PlugBoard { mapping: map }
    }
}
