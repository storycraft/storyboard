/*
 * Created on Sun Jul 16 2023
 *
 * Copyright (c) storycraft. Licensed under the Apache Licence 2.0.
 */

use std::{
    env,
    error::Error,
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use spx_codegen::{ext::FileExt, SpxBuilder};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=../workspace");

    let mut base = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    base.pop();

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let mut map_file = BufWriter::new(File::create(out_dir.join("resource_map"))?);
    let mut archive_file = BufWriter::new(File::create(
        base.join("target")
            .join(env::var("PROFILE")?)
            .join("resources.spx"),
    )?);

    let mut builder = SpxBuilder::new(&mut archive_file);

    builder.write_dir({
        let mut path = base.clone();
        path.push("workspace");

        path
    })?;

    write!(&mut map_file, "{}", builder.build())?;

    Ok(())
}
