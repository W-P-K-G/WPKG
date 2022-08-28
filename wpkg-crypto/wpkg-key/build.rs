fn main() {
    // create a random crypto key
    let x: u16 = rand::random();

    // set rustc env `CRYPTO_KEY`
    println!("cargo:rustc-env=CRYPTO_KEY={}", x);
}
