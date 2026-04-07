use rand::prelude::*;

pub fn get_rand_byte() -> u8 {
    let mut rng: ThreadRng = rand::rng();
    let random_byte: u8 = rng.random();
    random_byte
}
