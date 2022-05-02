use sync_file::SyncFile;
use zip::ZipArchive;

#[derive(Clone)]
pub struct ShellContext {
    pwd: String,
    pub reader: ZipArchive<SyncFile>,
}

impl ShellContext {
    pub fn new(path: String) -> Self {
        // TODO: Handle not found file
        let file = SyncFile::open(path).unwrap();

        Self {
            pwd: String::from("/"),
            reader: ZipArchive::new(file).unwrap(),
        }
    }

    pub fn pwd(&self) -> &String {
        &self.pwd
    }
    pub fn set_pwd(&mut self, new_pwd: String) {
        self.pwd = new_pwd;
    }
}
