use std::{
    borrow::Borrow,
    convert::Infallible,
    ffi::{OsStr, OsString},
    ops::Deref,
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct QDir {
    path_buf: PathBuf,
}

impl QDir {
    pub fn new() -> Self {
        QDir {
            path_buf: PathBuf::new(),
        }
    }

    pub fn into_path_buf(self) -> PathBuf {
        self.path_buf
    }
}

impl AsRef<OsStr> for QDir {
    fn as_ref(&self) -> &OsStr {
        self.path_buf.as_os_str()
    }
}

impl AsRef<Path> for QDir {
    fn as_ref(&self) -> &Path {
        self.path_buf.as_path()
    }
}

impl Borrow<Path> for QDir {
    fn borrow(&self) -> &Path {
        self.deref()
    }
}

impl Deref for QDir {
    type Target = Path;

    fn deref(&self) -> &Path {
        self.path_buf.as_path()
    }
}

impl<T: ?Sized + AsRef<OsStr>> From<&'_ T> for QDir {
    fn from(path: &T) -> Self {
        Self::from(path.as_ref().to_os_string())
    }
}

impl From<OsString> for QDir {
    fn from(path: OsString) -> Self {
        Self {
            path_buf: PathBuf::from(path),
        }
    }
}

impl From<QDir> for Box<Path> {
    fn from(q_dir: QDir) -> Box<Path> {
        q_dir.path_buf.into_boxed_path()
    }
}

impl From<QDir> for OsString {
    fn from(q_dir: QDir) -> OsString {
        q_dir.path_buf.into_os_string()
    }
}

impl From<String> for QDir {
    fn from(path: String) -> Self {
        Self {
            path_buf: PathBuf::from(path),
        }
    }
}

impl From<PathBuf> for QDir {
    fn from(path: PathBuf) -> Self {
        Self { path_buf: path }
    }
}

impl FromStr for QDir {
    type Err = Infallible;

    fn from_str(path: &str) -> Result<Self, Infallible> {
        Ok(Self {
            path_buf: PathBuf::from(path),
        })
    }
}
