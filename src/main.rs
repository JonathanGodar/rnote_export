use std::{
    fs::{self, create_dir_all},
    path::PathBuf,
    process::{Command, Stdio},
    time::{Duration, SystemTime},
};

use clap::{Parser, ValueHint};
use glob::glob;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(value_hint = ValueHint::DirPath)]
    directory: PathBuf,

    cache_dir: PathBuf,

    include: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let a = cli
        .directory
        .join(cli.include.unwrap_or("**".to_string()))
        .to_owned();

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
}
