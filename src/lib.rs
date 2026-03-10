use std::path::PathBuf;


/// Type representing a path that actually exists in the filesystem
/// (at the moment of creation of the ExistingPath object).
struct ExistingPath(PathBuf);

/// Type representing a file path that actually exists in the filesystem
/// (at the moment of creation of the ExistingFile object).
struct ExistingFile(PathBuf);

/// Type representing a directory path that actually exists in the filesystem
/// (at the moment of creation of the ExistingDir object).
struct ExistingDir(PathBuf);

/// Type representing a path that might not yet exist in the filesystem
/// but is consistent (at the moment of creation of the NewPath object),
/// meaning that no parent is an existing file (all parents are either existing directories,
/// or do not exist yet).
struct NewPath {
    path: PathBuf,
    root: PathBuf,
}

/// Type representing a file path that might not yet exist in the filesystem
/// but is consistent (at the moment of creation of the NewFile object),
/// meaning that no parent is an existing file (all parents are either existing directories,
/// or do not exist yet).
struct NewFile {
    path: PathBuf,
    root: PathBuf,
}

/// Type representing a directory path that might not yet exist in the filesystem
/// but is consistent (at the moment of creation of the NewDir object),
/// meaning that no parent is an existing file (all parents are either existing directories,
/// or do not exist yet).
struct NewDir {
    path: PathBuf,
    root: PathBuf,
}









#[cfg(test)]
mod tests {
    use super::*;

    
}
