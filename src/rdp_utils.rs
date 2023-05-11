use rand::Rng;

pub fn get_random_seq() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}