use clap::App;
use clap_generate::{generate_to, Generator};
use std::fs;
use std::path::PathBuf;

pub fn generate<T>(app: &App, dir: &PathBuf)
where
    T: Generator,
{
    if dir.exists() {
        fs::remove_dir_all(dir.clone()).unwrap();
    }

    let mut app = app.clone();

    fs::create_dir(dir.clone()).unwrap();
    generate_to::<T, _, _>(&mut app, env!("CARGO_PKG_NAME"), &dir);
}
