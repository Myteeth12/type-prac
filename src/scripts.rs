use rand::prelude::*;

pub fn word_random() -> &'static str {
    let mut rng = rand::rng();
    let rand_num: i8 = rng.random_range(0..=9);
    let word_list = [
        "that", "this", "how", "why", "even", "try", "shit", "tragic", "howdy", "dangle",
    ];
    word_list[rand_num as usize]
}
