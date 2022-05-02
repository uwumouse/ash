use sync_file::SyncFile;
use zip::ZipArchive;

#[derive(Clone)]
pub struct ShellContext {
    cwd: String,
    pub reader: ZipArchive<SyncFile>,
}

impl ShellContext {
    pub fn new(path: String) -> Self {
        // TODO: Handle not found file
        let file = SyncFile::open(path).unwrap();

        Self {
            cwd: String::from("/"),
            reader: ZipArchive::new(file).unwrap(),
        }
    }

    pub fn pwd(&self) -> &String {
        &self.cwd
    }
    pub fn set_cwd(&mut self, new_pwd: String) {
        self.cwd = new_pwd;
    }
}
