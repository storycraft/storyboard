/*
 * Created on Thu Jul 20 2023
 *
 * Copyright (c) storycraft. Licensed under the Apache Licence 2.0.
 */

use std::{collections::HashMap, path::PathBuf};

use spx::FileMap;

use super::SpxStorageSystem;

#[derive(Debug, Clone)]
pub struct Builder {
    map: HashMap<PathBuf, FileMap>
}

impl Builder {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn add(mut self, path: PathBuf, map: FileMap) -> Self {
        self.map.insert(path, map);

        self
    }

    pub fn build(self, path: PathBuf) -> SpxStorageSystem {
        SpxStorageSystem::from_map(path, self.map)
    }
}
