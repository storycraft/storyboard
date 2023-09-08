/*
 * Created on Thu Sep 07 2023
 *
 * Copyright (c) storycraft. Licensed under the Apache Licence 2.0.
 */

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=../workspace");
    
    Ok(())
}
