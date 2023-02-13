use diary::config::*;
use diary::db::*;
use diary::errors::BoxedErrorResult;

use dirs;
use std::{env, fs, io, path};
use structopt::StructOpt;

const DEFAULT_CONFIG: &str = ".config/diary/config.yaml";

#[derive(Debug, StructOpt)]
pub struct Add {
    #[structopt(short, long)]
    date: Option<String>,
    activity: String,
}

#[derive(Debug, StructOpt)]
pub struct Edit {
    #[structopt(short, long)]
    date: Option<String>,
    activity: String,
}

#[derive(Debug, StructOpt)]
pub struct Show {
    #[structopt(short, long)]
    first: Option<u32>,
    last: Option<u32>,
    date: Option<String>,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "diary")]
enum Command {
    #[structopt(name = "add")]
    Add(Add),
    #[structopt(name = "edit")]
    Edit(Edit),
    #[structopt(name = "show")]
    Show(Show),
}

fn add_diary_record(config: &Config, activity: &str, date: Option<String>) -> BoxedErrorResult<()> {
    for i in 0..config.len() {
        if config[i].name != activity {
            continue;
        }
        ensure_table_is_ready(&config[i])?;
        let answers = config[i]
            .fields
            .iter()
            .map(|f| {
                println!("{}", f.title);
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                input.to_owned()
            })
            .collect::<Vec<_>>();
        return save_diary_record(&config[i], answers);
    }

    println!("no such activity {}", activity);
    Ok(())
}

fn main() -> BoxedErrorResult<()> {
    let default_path = dirs::home_dir()
        .unwrap()
        .join(path::Path::new(DEFAULT_CONFIG));
    let config_path = env::var("DIARY_CONFIG_PATH").map_or(default_path, path::PathBuf::from);

    let f = fs::File::open(config_path)?;
    let config: Config = serde_yaml::from_reader(f).expect("cannot read backup metadata");

    let opt = Command::from_args();
    match opt {
        Command::Add(add) => add_diary_record(&config, &add.activity, add.date)?,
        Command::Edit(_) => (),
        Command::Show(_) => (),
    }

    Ok(())
}
