use rand::Rng;

fn main() {
    // generate a random key
    let mut rng = rand::thread_rng();
    let key = rng.gen_range(100..u16::MAX);

    // set rustc env `CRYPTO_KEY`
    println!("cargo:rustc-env=CRYPTO_KEY={}", key);
}
