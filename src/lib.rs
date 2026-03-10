use std::path::{Path, PathBuf};


/// Type representing a path that actually exists in the filesystem
/// (at the moment of creation of the ExistingPath object).
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExistingPath(PathBuf);

/// Type representing a file path that actually exists in the filesystem
/// (at the moment of creation of the ExistingFile object).
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExistingFile(PathBuf);

/// Type representing a directory path that actually exists in the filesystem
/// (at the moment of creation of the ExistingDir object).
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExistingDir(PathBuf);

/// Type representing a path that might not yet exist in the filesystem
/// but is consistent (at the moment of creation of the NewPath object),
/// meaning that no parent is an existing file (all parents are either existing directories,
/// or do not exist yet).
pub struct NewPath {
    path: PathBuf,
    root: PathBuf,
}

/// Type representing a file path that might not yet exist in the filesystem
/// but is consistent (at the moment of creation of the NewFile object),
/// meaning that no parent is an existing file (all parents are either existing directories,
/// or do not exist yet).
pub struct NewFile {
    path: PathBuf,
    root: PathBuf,
}

/// Type representing a directory path that might not yet exist in the filesystem
/// but is consistent (at the moment of creation of the NewDir object),
/// meaning that no parent is an existing file (all parents are either existing directories,
/// or do not exist yet).
pub struct NewDir {
    path: PathBuf,
    root: PathBuf,
}





impl AsRef<Path> for ExistingPath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl AsRef<Path> for ExistingFile {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl AsRef<Path> for ExistingDir {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl AsRef<Path> for NewPath {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

impl AsRef<Path> for NewFile {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

impl AsRef<Path> for NewDir {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    
}
