/*
 * Created on Fri Jul 07 2023
 *
 * Copyright (c) storycraft. Licensed under the Apache Licence 2.0.
 */

pub mod module_loader;
pub mod spx;

use std::{env, error::Error, rc::Rc, sync::Arc};

use crate::spx::builder::Builder as SpxStorageSystemBuilder;
use ::spx::FileMap;
use deno_core::ModuleSpecifier;
use deno_runtime::{
    permissions::PermissionsContainer,
    worker::{MainWorker, WorkerOptions},
    BootstrapOptions,
};
use module_loader::ModuleLoader;

const RESOURCE_MAP: FileMap = include!(concat!(env!("OUT_DIR"), "/resource_map"));

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut executable_dir = env::current_exe().unwrap();
    executable_dir.pop();

    let storage_system = SpxStorageSystemBuilder::new()
        .add("resources.spx".into(), RESOURCE_MAP)
        .build(executable_dir);

    let storage_system = Arc::new(storage_system);

    let main_module = ModuleSpecifier::parse("spx://resources.spx/main.js").unwrap();

    let mut worker = MainWorker::bootstrap_from_options(
        main_module.clone(),
        PermissionsContainer::allow_all(),
        WorkerOptions {
            extensions: vec![],
            bootstrap: create_boot_strap_options(),
            module_loader: Rc::new(ModuleLoader::new(storage_system.clone())),
            ..Default::default()
        },
    );

    worker.execute_main_module(&main_module).await?;
    worker.run_event_loop(false).await?;

    Ok(())
}

fn create_boot_strap_options() -> BootstrapOptions {
    BootstrapOptions {
        user_agent: format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),

        ..Default::default()
    }
}
