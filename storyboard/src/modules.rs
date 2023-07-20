/*
 * Created on Sun Jul 16 2023
 *
 * Copyright (c) storycraft. Licensed under the Apache Licence 2.0.
 */

use std::{future::Future, pin::Pin, sync::Arc};

use deno_core::{
    error::{generic_error, AnyError},
    futures::{future::ready, AsyncReadExt},
    FsModuleLoader, ModuleSource, ModuleSourceFuture, ModuleSpecifier, ModuleType,
    ResolutionKind,
};
use relative_path::RelativePath;

use crate::spx::SpxStorageSystem;

#[derive(Debug)]
pub struct ModuleLoader {
    storage: Arc<SpxStorageSystem>,
}

impl ModuleLoader {
    pub const fn new(storage: Arc<SpxStorageSystem>) -> Self {
        Self { storage }
    }

    fn load_spx(
        &self,
        module_specifier: ModuleSpecifier,
    ) -> impl Future<Output = Result<ModuleSource, AnyError>> {
        let storage = self.storage.clone();

        async move {
            let path = module_specifier
                .domain()
                .ok_or_else(|| generic_error("Spx archive name is not specified"))?;

            let file_name = {
                let path = module_specifier.path();

                &path[1.min(path.len())..]
            };

            let module_type = type_from_file_name(file_name);

            let mut code = String::new();

            storage
                .open(&path, &file_name)
                .await?
                .read_to_string(&mut code)
                .await?;

            Ok(ModuleSource::new(
                module_type,
                code.into(),
                &module_specifier,
            ))
        }
    }
}

impl deno_core::ModuleLoader for ModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        kind: ResolutionKind,
    ) -> Result<ModuleSpecifier, deno_core::anyhow::Error> {
        FsModuleLoader.resolve(specifier, referrer, kind)
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        maybe_referrer: Option<&ModuleSpecifier>,
        is_dyn_import: bool,
    ) -> Pin<Box<ModuleSourceFuture>> {
        match module_specifier.scheme() {
            "file" => FsModuleLoader.load(module_specifier, maybe_referrer, is_dyn_import),

            "spx" => Box::pin(self.load_spx(module_specifier.clone())),

            scheme => Box::pin(ready(Err(generic_error(format!(
                "Unknown scheme {}",
                scheme
            ))))),
        }
    }
}

fn type_from_file_name(file_name: &str) -> ModuleType {
    if let Some(extension) = RelativePath::new(&file_name).extension() {
        let ext = extension.to_lowercase();
        if ext == "json" {
            ModuleType::Json
        } else {
            ModuleType::JavaScript
        }
    } else {
        ModuleType::JavaScript
    }
}
