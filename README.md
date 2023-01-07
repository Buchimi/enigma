# Enigma machine in rust
This project implements the [Enigma Machine](https://en.wikipedia.org/wiki/Enigma_machine) in the Rust programming language. It uses a "default" configuration, meaning that it follows the specifications of the original Enigma machine as closely as possible.

The implementation allows stepping and double stepping, and the Enigma machine is designed to be modular, so it can work without needing all components.

## What is the Enigma Machine
I won't go into too much detail here so I recommend watching this video [This video](https://youtu.be/NDK78221OUk) for some background about the machine.

The enigma machine is a cipher that was used by the Nazi Millitary in WW2 to encrypt their messages.
In short, it's a glorified multilayered ceaser cipher that changes it's encryption setting every keystroke by "stepping" it's rotor and also sort of "encrypts backwards" after your input is partially encrypted.

Imagine a pipeline. 

char -> | swap | encrypt | encrypt | encrypt | reflect.
<ol>
<li> the char is swapped at the plugboard.  </li>
<li> encrypted several times by rotors.   </li>

(Though in each step. the result is fed into the next stage of the pip)
<li>
Then it's "reflected" which is fancy for it's swapped another time but now it has to go through the pipeline again but backwards
</li>
</ol>

result <- | swap | encrypt | encrypt | encrypt | <- reflected char

and every character fed into this pipeline would go through different settings in the "encrypt" stages. This change of setting is called a rotor step

It is also important to know that you get the same letter if you "encrypt" the encrypted string with the same setting

## My reasons
- It wanted to challenge myself as i had to research and implement the engima machine with zero background knowledge before spring break ended (spoiler, I did it in 2-3 days 😎)
- It's a somewhat unique project
- Wanted to use rust

# What's next
I'll like to figure out how to deploy this on the web for easy use. Using something like webassembly.
Also make it so that you can change the setting easily.

#Installation

To use this project, you need to have Rust installed on your system.

To install Rust, please follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install)

Once Rust is installed, clone this repository and build the project by running:

```sh
cargo build
```

# Usage

To use the Enigma machine, you need to provide the following parameters:

- The message to be encrypted or decrypted
- The initial rotor positions (also called "ring settings")
- The plugboard settings (optional)
Here is an example of how to use the Enigma machine to encrypt a message:

```rust
use enigma_machine::{Enigma, RotorPositions, Plugboard};

let message = "HELLO WORLD";

let enigma = Enigma::generic_enigma();
let encrypted = enigma.encrypt_string(message.to_string());

println!("Encrypted message: {}", encrypted);
```
To decrypt a message, reset the enigma by calling 
```rust
let new_enigma = enigma.reset()
```
and passing in the encrypted messase using the same encrypt string function.



## Thanks for taking a look at my github.
<b> And have a wonderful day </b>
