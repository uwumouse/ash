use crate::shell_context::ShellContext;

// TODO
pub struct ZipPath {
    path: String,
    is_dir: bool,
}

// TODO:
// 1. new
//  - normalize the path when creating
// 2. is_parent_of
// 3. is_dir
// 4. has_children
impl ZipPath {
    /// Creates a ZipPath sanitazing the path:
    /// - Makes it absolute (without / at the start, like in ZIP)
    /// - Add slashes at the end of the directories
    pub fn new(path: String, ctx: &mut ShellContext) -> Self {
        unimplemented!();
    }

    /// Creates a ZipPath assuming that `path` is already in ZipFormat
    pub fn from_normalized(path: String) -> Self {
        unimplemented!();
    }

    /// Wether this path is a part of other path
    /// ! works only for directories
    pub fn is_parent_of(&self, path: String) -> bool {
        if !self.is_dir {
            return false;
        }
        unimplemented!();
    }

    // TODO: Do I even need this?
    /// Returns all children that belong to this
    //pub fn get_children(&self, ctx: &mut ShellContext) -> Result<Vec<Self>,String> {
    //if !self.is_dir {
    //return Err("Files cannot have children");
    //}

    //unimplemented!();
    //}

    /// Returns name of the file/dir that this path represents
    pub fn name() -> String {
        unimplemented!();
    }
}
