use std::{
    fs::{self, create_dir_all, write},
    path::PathBuf,
    process::{Command, Stdio},
    time::{Duration, SystemTime},
};

use clap::{Parser, Subcommand, ValueHint};
use glob::glob;
use serde::{Deserialize, Serialize};
use toml::value::Time;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(value_hint = ValueHint::DirPath)]
    directory: PathBuf,
    include: String,

    cache_dir: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    //cli.directory = cli.directory.canonicalize().unwrap();
    //cli.cache_dir = cli.cache_dir.canonicalize().unwrap();

    let cli = dbg!(cli);

    let a = cli.directory.join(cli.include).to_owned();
    let entries = glob(a.to_str().unwrap()).expect("Shit hit the fan");

    println!("{:?}", entries.count());

    for entry in glob(a.to_str().unwrap()).expect("Shit hit the fan") {
        match entry {
            Ok(path) => {
                let mut of = cli
                    .cache_dir
                    .join(path.strip_prefix(&cli.directory).unwrap());

                of.set_extension("png");

                // Don't create file if it was last modified less then an hour ago => likely to
                // change again
                let if_modified = fs::metadata(&path).unwrap().modified().unwrap();
                if if_modified > SystemTime::now() - Duration::from_secs(60 * 60) {
                    println!(
                        "Skipping {:?} because it was modified less than an hour ago",
                        path
                    );
                    continue;
                }

                if let Ok(meta) = fs::metadata(of.clone()) {
                    let of_modified = meta.modified().unwrap();

                    // Only update if the input file was modified more recently than what the
                    // output file was
                    if of_modified > if_modified {
                        println!("Skipping {:?} because it has not changed", path);
                        continue;
                    }
                }

                println!("{:?}", of.parent().unwrap());
                create_dir_all(of.parent().unwrap()).unwrap();
                let mut cmd = Command::new("rnote-cli")
                    .args([
                        "export",
                        "selection",
                        "--output-file",
                        of.to_str().unwrap(),
                        "all",
                        //"-overwrite",
                        path.to_str().unwrap(),
                    ])
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .unwrap();

                cmd.wait().unwrap();
            }
            Err(_) => todo!(),
        }
    }

    //let pase = BringCollection {
    //    sakes: vec![Bring {
    //        name: "rust cargo".to_string(),
    //        pryles: vec![BringFile {
    //            file_path: "./flake.nix".into(),
    //            create_option: CreateOption::Quit,
    //        }],
    //    }],
    //};
    //
    //write("pase.toml", toml::to_string(&pase).unwrap()).unwrap();
    //
    //let pb = PathBuf::from("./hejsan");
    //
    //dbg!(pb);
    //println!("Hello, world!");
}
