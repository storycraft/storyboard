/*
 * Created on Mon Jul 17 2023
 *
 * Copyright (c) storycraft. Licensed under the Apache Licence 2.0.
 */

use std::{
    error::Error,
    ffi::OsStr,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
    process::Command,
};

use spx_codegen::{ext::FileExt, SpxBuilder};

pub fn dist<S: AsRef<OsStr>>(
    cargo_args: impl IntoIterator<Item = S>,
) -> Result<(), Box<dyn Error>> {
    let base = Path::new(env!("CARGO_MANIFEST_DIR"));
    let out_dir = Path::new(env!("OUT_DIR"));

    let resource_map_path = out_dir.join("resource_map");
    let archive_file_path = out_dir.join("resources.spx");

    {
        let mut map_file = BufWriter::new(File::create(&resource_map_path)?);
        let mut archive_file = BufWriter::new(File::create(&archive_file_path)?);

        let mut builder = SpxBuilder::new(&mut archive_file);

        builder.write_dir({
            let mut path = base.to_path_buf();
            path.pop();
            path.push("workspace");

            path
        })?;

        write!(&mut map_file, "{}", builder.build())?;
    }

    Command::new("cargo")
        .env("SPX_OUT", "spx")
        .env("STORYBOARD_RESOURCE_MAP", &resource_map_path)
        .env("STORYBOARD_ENTRYPOINT", "spx://resources.spx/main.js")
        .args(["build", "--package", "storyboard"])
        .args(cargo_args)
        .status()?;

    Ok(())
}
