use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct QDir {
    path_buf: PathBuf,
}

impl<'a> QDir {
    pub fn new() -> Self {
        QDir {
            path_buf: PathBuf::new(),
        }
    }

    pub fn into_path_buf(&'a self) -> &'a PathBuf {
        &self.path_buf
    }
}

impl AsRef<Path> for QDir {
    fn as_ref(&self) -> &Path {
        self.path_buf.as_path()
    }
}

impl From<PathBuf> for QDir {
    fn from(path: PathBuf) -> Self {
        Self { path_buf: path }
    }
}

impl From<&Path> for QDir {
    fn from(path: &Path) -> Self {
        Self {
            path_buf: PathBuf::from(path),
        }
    }
}
