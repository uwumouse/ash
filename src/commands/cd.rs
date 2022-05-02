use super::Command;
use crate::{shell_context::ShellContext, utils};
use zip::result::ZipError;
pub struct CdCommand;

impl Command for CdCommand {
    fn name(&self) -> &'static str {
        "cd"
    }

    fn exec(&self, args: Vec<&str>, ctx: &mut ShellContext) -> Result<(), String> {
        if args.len() == 1 {
            ctx.set_pwd(String::from("/"));
        } else {
            let mut new_cwd = utils::normalize_zip_path(args[1]);
            if !new_cwd.ends_with("/") {
                new_cwd.push('/');
            }
            let mut reader = ctx.reader.clone();
            let f = reader.by_name(&new_cwd);

            match f {
                Ok(f) => {
                    if f.is_dir() {
                        ctx.set_pwd(utils::normalize_zip_path(f.name()));
                    } else {
                        eprintln!("Cannot cd into {}: it's a file", f.name());
                    }
                }
                Err(e) => {
                    match e {
                        ZipError::FileNotFound => eprintln!("Directory not found."),
                        _ => {}
                    };
                }
            }
        }

        Ok(())
    }
}
