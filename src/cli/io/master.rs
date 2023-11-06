use std::{fs::File, io::Read};

use super::statics::APP_DIR_CONTENTS;

pub fn get_master_password() -> anyhow::Result<Option<String>> {
    let master_file = APP_DIR_CONTENTS.first();

    if master_file.is_none() {
        return Ok(None);
    }

    let file = master_file
        .as_ref()
        .unwrap()
        .as_ref()
        .map_err(anyhow::Error::new)?
        .path();

    let mut file = File::open(file)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    Ok(Some(buf))
}
