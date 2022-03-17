mod plugboard;
mod reflector;
mod rotor;

use plugboard::PlugBoard;
use reflector::Reflector;
use rotor::Rotor;
pub struct Enigma {
    rotor1: Rotor,
    rotor2: Rotor,
    rotor3: Rotor,
    reflector: Reflector,
    plugboard: PlugBoard,
}

impl Enigma {
    #[allow(dead_code)]
    pub fn generic_enigma() -> Enigma {
        Enigma {
            rotor1: Rotor::new_rotor_I(0, 'A'),
            rotor2: Rotor::new_rotor_II(0, 'A'),
            rotor3: Rotor::new_rotor_III(0, 'A'),
            reflector: Reflector::reflector_b(),
            plugboard: PlugBoard::new(),
        }
    }
    #[allow(dead_code)]
    pub fn encrypt_string(&mut self, string: String) -> String {
        let mut str = String::new();
        for char in string.chars() {
            str += self.encrypt_char(char).to_string().as_str();
        }
        return str;
    }
    #[allow(dead_code)]
    pub fn encrypt_char(&mut self, character: char) -> char {
        let mut ret = self.plugboard.swap(character);
        self.step_rotors();
        println!(
            "Starting pos for rotor 1 {}, rotor 2 {}, rotor 3 {}",
            (self.rotor1.get_step() + 65) as char,
            (self.rotor2.get_step() + 65) as char,
            (self.rotor3.get_step() + 65) as char
        );
        println!("{}", ret);
        ret = self.rotor3.encrypt(ret) as char;
        println!("{}", ret);
        ret = self.rotor2.encrypt(ret) as char;
        println!("{}", ret);
        ret = self.rotor1.encrypt(ret) as char;
        println!("{}", ret);
        ret = self.reflector.reflect(ret);
        println!("{}", ret);
        ret = self.rotor1.reverse_crypt(ret) as char;
        println!("{}", ret);
        ret = self.rotor2.reverse_crypt(ret) as char;
        println!("{}", ret);
        ret = self.rotor3.reverse_crypt(ret) as char;
        println!("{}", ret);
        ret = self.plugboard.swap(ret);
        println!("{}", ret);
        println!("done");
        ret
    }
    #[allow(dead_code)]
    pub fn reset(self) -> Enigma {
        Enigma::generic_enigma()
    }
    pub fn step_rotors(&mut self) {
        self.rotor3.step();
        if self.rotor2.get_step() + 1 == self.rotor2.get_notch_pos() {
            //this should be a double step
            //step
            self.rotor2.step();
            self.rotor1.step();
        } else if self.rotor3.get_step() == self.rotor3.get_notch_pos() {
            self.rotor2.step();
        }

        // else if self.rotor1.get_step() == self.rotor1.get_notch_pos() {
        //     //CONDITION FOR DOUBLE STEP
        //     self.rotor2.step();
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_enigma() {
        let mut enigma = Enigma::generic_enigma();
        assert_eq!(enigma.encrypt_char('A'), 'M');
    }
    #[test]
    fn test_string_encryption() {
        let mut enigma = Enigma::generic_enigma();
        assert_eq!(enigma.encrypt_string("AAAAA".to_string()), "MDVFG");
        let mut another_enigma = Enigma::generic_enigma();
        assert_eq!(another_enigma.encrypt_string("ABCDE".to_string()), "MVOIW");
    }
    #[test]
    fn test_multistep() {
        let mut enigma = Enigma::generic_enigma();
        enigma.rotor1.set_offset_and_start_pos(0, 'A');
        enigma.rotor2.set_offset_and_start_pos(0, 'E');
        enigma.rotor3.set_offset_and_start_pos(0, 'V');
        assert_eq!(enigma.encrypt_string("AA".to_string()), "KT");
    }
    #[test]
    fn another_multistep_test() {
        let mut enigma = Enigma::generic_enigma();
        enigma.rotor1.set_offset_and_start_pos(0, 'Q');
        enigma.rotor2.set_offset_and_start_pos(0, 'D');
        enigma.rotor3.set_offset_and_start_pos(0, 'U');
        assert_eq!(enigma.encrypt_string("AAA".to_string()), "TIG");
        enigma = enigma.reset();
        enigma.rotor1.set_offset_and_start_pos(0, 'Q');
        enigma.rotor2.set_offset_and_start_pos(0, 'D');
        enigma.rotor3.set_offset_and_start_pos(0, 'U');
        assert_eq!(enigma.encrypt_string("TIG".to_string()), "AAA");
    }
}
