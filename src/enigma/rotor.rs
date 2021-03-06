//Go to
//https://www.cryptool.org/en/cto/enigma-step-by-step
//For a better experience
//not my website by the way!!!

#[allow(dead_code)]
pub struct Rotor {
    output_map: Vec<char>,

    notch_pos: u8,
    step: u8,   //A step of 0 = A starting positon of A
    offset: u8, //An offset of 0 = no ring shifting
}
impl Rotor {
    #[allow(dead_code)]
    pub fn set_offset_and_start_pos(&mut self, offset: u8, starting_position: char) {
        self.offset = offset;
        self.step = starting_position as u8 - 65;
    }

    #[allow(dead_code)]
    pub fn step(&mut self) -> &mut Rotor {
        self.step = (self.step + 1) % 26;
        self
    }
    #[allow(dead_code)]
    pub fn get_step(&self) -> u8 {
        self.step
    }

    #[allow(dead_code)]
    pub fn get_notch_pos(&self) -> u8 {
        self.notch_pos
    }
    //These are standardized rotors
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn new_rotor_I(offset: u8, starting_position: char) -> Rotor {
        Rotor {
            output_map: "EKMFLGDQVZNTOWYHXUSPAIBRCJ".chars().collect(), //map(|chr| chr.to_string()) //.collect(),
            notch_pos: 'R' as u8 - 65,
            step: starting_position as u8 - 65,
            offset: offset,
        }
    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn new_rotor_II(offset: u8, starting_position: char) -> Rotor {
        Rotor {
            output_map: "AJDKSIRUXBLHWTMCQGZNPYFVOE".chars().collect(),
            notch_pos: 'F' as u8 - 65,
            step: starting_position as u8 - 65,
            offset: offset,
        }
    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn new_rotor_III(offset: u8, starting_position: char) -> Rotor {
        Rotor {
            output_map: "BDFHJLCPRTXVZNYEIWGAKMUSQO".chars().collect(),
            notch_pos: 'W' as u8 - 65,
            step: starting_position as u8 - 65,
            offset: offset,
        }
    }
    //Done with standard rotors

    //Now we will create a custom rotor
    #[allow(dead_code)]
    fn new_custom_rotor(string: &str, notch: u8, offset: u8, starting_position: char) -> Rotor {
        assert_eq!(string.chars().count(), 26); //Is the length of the string 26?
        Rotor {
            output_map: string.chars().collect(),
            notch_pos: notch,
            step: starting_position as u8 - 65,
            offset: offset,
        }
    }
    #[allow(dead_code)]
    pub fn encrypt(&mut self, letter: char) -> u8 {
        let index = letter as u8 - 65;
        //Encryption using the step

        //first we gotta step tho
        //what we want to do is,
        //find char at step + index
        let total_shift = (self.step) as i8 - (self.offset as i8);
        let perm_char = self.output_map[((index as i8 + total_shift + 26) % 26) as usize];
        //^^ this is how we permutate
        //turn that char to an integer/uint
        //subtract by step to get the right excryption
        (((((perm_char as usize - 65) as i8 - total_shift) + 26) % 26) + 65) as u8
        //this is assuming we have no offset

        //for offset, it's basically the same thing with step but reverse
        //index - offset
        //permutate
        //answer + offset
        //this is assuming a step of 0 or in other words, a starting position of Z
    }
    pub fn reverse_crypt(&mut self, letter: char) -> u8 {
        let mut letter_as_num = letter as u8;
        let total_shift = (self.step) as i8 - (self.offset as i8);
        letter_as_num = ((((letter_as_num - 65) as i8 + total_shift + 26) % 26) + 65) as u8;

        let index = self
            .output_map
            .iter()
            .position(|char| *char == letter_as_num as char);
        (((index.unwrap()) as i8 - total_shift + 26) % 26 + 65) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encryption() {
        assert_eq!(Rotor::new_rotor_III(0, 'A').step().encrypt('D'), 'I' as u8);
        assert_eq!(Rotor::new_rotor_II(0, 'A').encrypt('I'), 'X' as u8);
    }
    #[test]
    fn test_reverse_encryption() {
        assert_eq!(
            Rotor::new_rotor_I(4, 'A').step().reverse_crypt('C'),
            'M' as u8
        );
    }
}
