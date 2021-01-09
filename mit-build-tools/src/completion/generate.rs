use std::fs;
use std::path::Path;

use clap::App;
use clap_generate::{generate_to, Generator};

pub fn generate<T>(app: &App, dir: &Path)
where
    T: Generator,
{
    if dir.exists() {
        fs::remove_dir_all(dir).unwrap();
    }

    let name = app.get_bin_name().unwrap();
    let mut app = app.clone();

    fs::create_dir(dir.to_path_buf()).unwrap();
    generate_to::<T, _, _>(&mut app, name, &dir);
}
