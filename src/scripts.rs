use rand::prelude::*;

pub fn word_random() -> &'static str {
    let mut rng = rand::rng();
    let rand_num: i8 = rng.random_range(0..=103);
    let word_list = [
        // A
        "apple", "angle", "agent", "arise", // B
        "brave", "brick", "blame", "bloom", // C
        "crisp", "climb", "chase", "cloud", // D
        "drift", "dance", "draft", "dream", // E
        "eagle", "event", "enter", "empty", // F
        "flame", "frame", "feast", "frown", // G
        "grape", "glide", "ghost", "gloom", // H
        "hover", "hatch", "honey", "habit", // I
        "index", "inbox", "image", "ivory", // J
        "jelly", "joint", "judge", "jolly", // K
        "knife", "knock", "karma", "kitty", // L
        "latch", "lemon", "lunar", "lobby", // M
        "mango", "march", "motel", "magic", // N
        "noble", "nudge", "nasty", "novel", // O
        "orbit", "ocean", "occur", "onion", // P
        "piano", "pride", "plant", "punch", // Q
        "quake", "queen", "quest", "quick", // R
        "risky", "raven", "rebel", "rough", // S
        "spice", "sword", "shady", "silly", // T
        "tiger", "track", "taste", "torch", // U
        "ultra", "uncle", "urban", "upset", // V
        "vapor", "vital", "vivid", "vouch", // W
        "witty", "wrist", "worry", "weird", // X
        "xenon", "xerox", "xylem", "xenial", // Y
        "yacht", "yield", "young", "yearn", // Z
        "zebra", "zesty", "zonal", "zoomy",
    ];
    word_list[rand_num as usize]
}
