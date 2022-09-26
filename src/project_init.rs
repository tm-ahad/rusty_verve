use std::fs::{create_dir, File};
use std::io::Write;
use colored::Colorize;

pub fn init(name: String)
{
    let form = format!("[package]\nname = '{}'\n\
                    version = '0.1.0'\n\
                    edition = '2021'\n\
                    {}[dependencies]", name, "\n");
    let gi = "/target\n/.idea\n/.vscode\n/build_rs";
    let conf = "[config]\n\npath = './build'\nstart = 'cargo run'\n\n[modules]";

    let byte = form.as_str().as_bytes();

    create_dir(format!("./{}", name)).expect("TODO: panic message");
    create_dir(format!("./{}/build_rs", name)).expect("TODO: panic message");
    create_dir(format!("./{}/build_rs/src", name)).expect("TODO: panic message");
    create_dir(format!("./{}/src", name)).expect("TODO: panic message");

    let mut toml_file = File::create(format!("./{}/build_rs/Cargo.toml", name))
        .unwrap();
    let mut git_ignore = File::create(format!("./{}/.gitignore", name))
        .unwrap();
    let mut config = File::create(format!("./{}/config.toml", name))
        .unwrap();
    File::create(format!("./{}/src/main.verve", name))
        .unwrap();

    let mut mods = File::create(format!("./{}/build_rs/mods.toml", name))
        .unwrap();

    mods.write_all("[modules]".as_bytes()).expect("TODO: panic message");

    toml_file.write_all(byte).expect("TODO: panic message");
    git_ignore.write_all(gi.as_bytes()).expect("TODO: panic message");
    config.write_all(conf.as_bytes()).expect("TODO: panic message");

    println!("{}", "Project created.".green());
}