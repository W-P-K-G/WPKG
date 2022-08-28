fn main() {
    #[cfg(windows)]
    icon();
}

#[cfg(windows)]
fn icon() {
    let mut res = winres::WindowsResource::new();

    res.set_icon("assets/favicon.ico");
    res.compile().unwrap();
}
