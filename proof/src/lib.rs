use std::{
    fs::{create_dir, File},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};

pub mod prove;
pub mod verify;

const OUTPUT_DIR_NAME: &str = "output";

pub fn output_path(filename: &str) -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;
    let current_dir = current_dir
        .to_str()
        .ok_or(anyhow!("output_dir error (current_dir)"))?;

    let crate_path = "/proof";
    let workspace_root = current_dir
        .strip_suffix(crate_path)
        .or(Some(current_dir))
        .ok_or(anyhow!("output_dir error (crate_path)"))?;

    let mut output = Path::new(workspace_root).join(OUTPUT_DIR_NAME);

    // create the output dir if it doesn't exist
    if !output.exists() {
        create_dir(output.clone())?;
    }

    output.push(filename);

    Ok(output)
}

pub fn save_to_file(data: &[u8], filename: &str) -> Result<()> {
    let out = output_path(filename)?;
    File::create(out)?.write_all(data)?;
    Ok(())
}
