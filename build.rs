fn main() {
    // create a random crypto key
    let x: u16 = rand::random();

    println!("cargo:rustc-env=CRYPTO_KEY={}", x);

    #[cfg(windows)]
    icon();
}

#[cfg(windows)]
fn icon() {
    let mut res = winres::WindowsResource::new();

    res.set_icon("assets/favicon.ico");
    res.compile().unwrap();
}
