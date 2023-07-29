use std::{fs, path::Path};

mod generate_static_page;

fn main() {
    let _ = fs::write(Path::new("test.html"), generate_static_page::generate_static_page());
}

