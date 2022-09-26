mod project_init;
mod unknown_key;
mod error;
mod vec_str_concat;

use std::{env, fs};
use std::fs::File;
use std::io::Write;
use colored::Colorize;
use crate::project_init::init;
use crate::unknown_key::unknown_key;
use crate::vec_str_concat::vec_str_concat;

fn main() 
{
    let args: Vec<String> = env::args().collect();

    let com = args.get(1);
    match com 
    {
        Some(v) =>
            {

                if v == &("create".to_string()) {
                    init(args[2].clone());
                }

                if v == &("build".to_string())
                {

                    let mut rs = File::create("./build_rs/src/main.rs").unwrap();

                    let vr_iter: String = fs::read_to_string("./src/main.verve")
                        .expect("Should have been able to read the file");

                    let vr: Vec<&str> = vr_iter
                        .as_str()
                        .split("\n")
                        .collect();

                    let mut idx = 0 as u32;

                    for mut lines in vr.clone().into_iter()
                    {
                        unknown_key(vec!["->", "let"], lines.to_string(), idx, lines);
                        let l = lines.clone().replace("var", "let");
                        lines = &*l;

                        idx += 1;
                    }

                    rs.write_all(vec_str_concat(vr.clone()).as_bytes()).expect("TODO: panic message");

                    println!("{}\n          ", "Project Restored".bold().yellow());
                    println!("{}", "Success".green());
                }
            },
        None => println!("{}", "No command provided".red()),
    }
}
