mod plugboard;
mod reflector;
pub mod rotor;

use reflector::Reflector;
use rotor::Rotor;
pub struct Enigma {
    rotor1: Rotor,
    rotor2: Rotor,
    rotor3: Rotor,
    reflector: Reflector,
}
