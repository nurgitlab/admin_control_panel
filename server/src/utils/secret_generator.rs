use rand::Rng;

pub struct SecretGenerator;

impl SecretGenerator {
    // Random 6 digit code generator
    pub fn generate_numeric_code() -> String {
        let mut rng = rand::rng();
        (0..6).map(|_| rng.random_range(0..=9).to_string()).collect()
    }

    pub fn generate_alphanumeric_code(n: usize) -> String {
        if n == 0 {
            return String::new();
        }

        let mut rng = rand::rng();
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789";

        (0..n)
            .map(|_| {
                let idx = rng.random_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }
}
