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
        ret = self.rotor3.step().encrypt(ret) as char;
        ret = self.rotor2.encrypt(ret) as char;
        ret = self.rotor1.encrypt(ret) as char;
        ret = self.reflector.reflect(ret);
        ret = self.rotor1.reverse_crypt(ret) as char;
        ret = self.rotor2.reverse_crypt(ret) as char;
        ret = self.rotor3.reverse_crypt(ret) as char;
        ret = self.plugboard.swap(ret);
        ret
    }
    #[allow(dead_code)]
    pub fn reset(self) -> Enigma {
        Enigma::generic_enigma()
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
}
