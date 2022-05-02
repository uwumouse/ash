use super::Command;
use crate::{shell_context::ShellContext, utils};
use zip::{read::ZipFile, result::ZipError};

pub struct LsCommand;

impl Command for LsCommand {
    fn name(&self) -> &'static str {
        "ls"
    }

    fn exec(&self, args: Vec<&str>, ctx: &mut ShellContext) -> Result<(), String> {
        let path = if args.len() > 1 { args[1] } else { ctx.pwd() };
        let mut path = path
            .trim_start_matches("/")
            .trim_end_matches("/")
            .to_string();

        // TODO: List root
        if path == "" {
            return self.list_files(ctx, path);
        }

        if let Some(f) = self.get_file(ctx, &path)? {
            let (size, unit) = utils::human_readable_size(f.size());
            println!("(f) {} | {}{} (uncompressed).", f.name(), size, unit);
            return Ok(());
        }

        path.push('/');
        let mut ctx1 = ctx.clone();
        let dir = self.get_file(&mut ctx1, &path)?;

        if dir.is_none() {
            return Err(format!("File or Directory `{}` not found.", path));
        }

        let dir = dir.unwrap();

        return self.list_files(&mut ctx.clone(), dir.name().to_string());
    }
}

impl<'a> LsCommand {
    fn get_file(
        &self,
        ctx: &'a mut ShellContext,
        path: &str,
    ) -> Result<Option<ZipFile<'a>>, String> {
        match ctx.reader.by_name(path) {
            Ok(f) => Ok(Some(f)),
            Err(e) => match e {
                ZipError::FileNotFound => Ok(None),
                _ => Err(format!("Unknown error: {}", e)),
            },
        }
    }

    fn list_files(&self, ctx: &mut ShellContext, path: String) -> Result<(), String> {
        // Finds file in given file and returns (absolute, relative) paths
        // Amound of directories in path to current directory
        let p_dirs = path.split('/').collect::<Vec<&str>>().len();
        let files = ctx
            .reader
            .file_names()
            .filter(|f| {
                // All parents of current file
                let parents = f.split('/').collect::<Vec<&str>>();

                // File is considered relative when amount of directories in path to it is less or
                // equal to amount of directoires in path to parent.
                // It can be +1 when the file is directory itself
                if f.starts_with(&path) {
                    if parents.len() == p_dirs {
                        return true;
                    } else if parents.len() <= p_dirs + 1 && parents.last().unwrap() == &"" {
                        return true;
                    }
                }

                false
            })
            .map(|f| {
                if f == path {
                    return (f, ".");
                }

                let rel = f.trim_start_matches(&path).trim_start_matches("/");
                return (f, rel);
            })
            .collect::<Vec<(&str, &str)>>();

        let mut entries: Vec<String> = vec![];
        for (abs, rel) in files {
            let ctx = &mut ctx.clone();
            let file = self.get_file(ctx, abs)?;
            if let Some(f) = file {
                let ftype = if f.is_file() { 'f' } else { 'd' };
                let (size, unit) = utils::human_readable_size(f.size());
                let entry = format!("({}) {} | {}{} (uncompressed)", ftype, rel, size, unit);
                entries.push(entry);
            }
        }

        entries.sort();
        for e in entries {
            println!("{}", e);
        }

        Ok(())
    }
}
