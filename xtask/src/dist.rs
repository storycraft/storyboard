/*
 * Created on Mon Jul 17 2023
 *
 * Copyright (c) storycraft. Licensed under the Apache Licence 2.0.
 */

use std::{error::Error, ffi::OsStr, process::Command};

pub fn dist<S: AsRef<OsStr>>(
    cargo_args: impl IntoIterator<Item = S>,
) -> Result<(), Box<dyn Error>> {
    Command::new("cargo")
        .env("SPX_OUT", "spx")
        .args(["build", "--package", "storyboard"])
        .args(cargo_args)
        .status()?;

    Ok(())
}
