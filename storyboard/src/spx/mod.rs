/*
 * Created on Wed Jul 19 2023
 *
 * Copyright (c) storycraft. Licensed under the Apache Licence 2.0.
 */

pub mod builder;

use std::{
    collections::HashMap,
    io,
    path::{Path, PathBuf},
};

use relative_path::RelativePath;
use spx::{
    io::{SpxArchive, SpxAsyncFileStream},
    FileMap,
};
use tokio::{fs::File, io::BufReader};
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};

pub type FsSpxArchive<'a> = SpxArchive<'a, Compat<BufReader<File>>>;
pub type FsSpxAsyncFileStream<'a> = SpxAsyncFileStream<Compat<BufReader<File>>>;

#[derive(Debug)]
pub struct SpxStorageSystem {
    root: PathBuf,
    map: HashMap<PathBuf, FileMap>,
}

impl SpxStorageSystem {
    pub fn new(root: PathBuf) -> Self {
        Self::from_map(root, HashMap::new())
    }

    pub fn from_map(root: PathBuf, map: HashMap<PathBuf, FileMap>) -> Self {
        Self { root, map }
    }

    pub async fn root(&self) -> &Path {
        &self.root
    }

    pub async fn open_archive(&self, path: &impl AsRef<Path>) -> io::Result<FsSpxArchive<'_>> {
        let path = path.as_ref();

        let map = self.map.get(path).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::AddrNotAvailable,
                format!("Archive `{}` is not registered", path.display()),
            )
        })?;

        Ok(SpxArchive::new(
            map,
            BufReader::new(File::open(self.root.join(path)).await?).compat(),
        ))
    }

    pub async fn open(
        &self,
        path: &impl AsRef<Path>,
        name: &impl AsRef<RelativePath>,
    ) -> io::Result<FsSpxAsyncFileStream> {
        Ok(self
            .open_archive(path)
            .await?
            .open_async(name.as_ref())
            .await?)
    }
}
