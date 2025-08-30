use rand::Rng;

pub struct SecretGenerator;

impl SecretGenerator {
    // Random 6 digit code generator
    pub fn generate_numeric_code() -> String {
        let mut rng = rand::rng();
        (0..6).map(|_| rng.random_range(0..=9).to_string()).collect()
    }
}
