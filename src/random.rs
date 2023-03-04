use rand::Rng;

pub fn random(min: &f64, max: &f64) -> f64 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(*min..*max) as f64;
}