use crate::{
    commands::{CdCommand, Command, LsCommand},
    shell_context::ShellContext,
};
use colored::*;
use std::io::{self, Write};

pub struct Shell {
    ctx: ShellContext,
    cmds: Vec<Box<dyn Command>>,
}

impl Shell {
    pub fn new(arc_path: String) -> Self {
        let cmds: Vec<Box<dyn Command>> = vec![Box::new(LsCommand), Box::new(CdCommand)];

        Self {
            cmds,
            ctx: ShellContext::new(arc_path),
        }
    }

    pub fn start(&mut self) {
        loop {
            let input = self.prompt();
            let mut cmd_found = false;
            for i in &self.cmds {
                if input.starts_with(i.name()) {
                    let res = i.exec(input.split(" ").collect(), &mut self.ctx);
                    if let Err(e) = res {
                        eprintln!("ash: {}", e);
                    }
                    cmd_found = true;
                    break;
                }
            }

            if !cmd_found {
                println!("ash: Command `{}` not found.", input);
            }
        }
    }

    fn prompt(&self) -> String {
        print!("[{}]$ ", self.ctx.pwd().blue());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input.");

        return input.trim_end().to_string();
    }
}
