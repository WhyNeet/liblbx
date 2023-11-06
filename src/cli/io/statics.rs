use std::{
    fs::{self, DirEntry},
    io, process,
};

use lazy_static::lazy_static;

use super::app_dir;

lazy_static! {
    pub static ref APP_DIR_CONTENTS: Vec<io::Result<DirEntry>> = read_app_dir();
}

fn read_app_dir() -> Vec<io::Result<DirEntry>> {
    let app_dir = app_dir::get_app_dir().unwrap_or_else(|err| {
        eprintln!("A fatal error occured: {err}");
        process::exit(1)
    });

    fs::read_dir(app_dir)
        .unwrap_or_else(|err| {
            eprintln!("A fatal error occured: {err}");
            process::exit(1)
        })
        .collect()
}
