use diary::config::*;
use diary::errors::BoxedErrorResult;

use dirs;
use std::{env, fs, path};
use structopt::StructOpt;

const DEFAULT_CONFIG: &str = ".config/diary/config.yaml";

#[derive(Debug, StructOpt)]
pub struct Add {
    #[structopt(short, long)]
    date: Option<String>,
    actitbity: String,
}

#[derive(Debug, StructOpt)]
pub struct Edit {
    #[structopt(short, long)]
    date: Option<String>,
    actitbity: String,
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

fn main() -> BoxedErrorResult<()> {
    let default_path = dirs::home_dir()
        .unwrap()
        .join(path::Path::new(DEFAULT_CONFIG));
    let config_path = env::var("DIARY_CONFIG_PATH").map_or(default_path, path::PathBuf::from);

    let f = fs::File::open(config_path)?;
    let config: Config = serde_yaml::from_reader(f).expect("cannot read backup metadata");

    let opt = Command::from_args();
    match opt {
        Command::Add(_) => (),
        Command::Edit(_) => (),
        Command::Show(_) => (),
    }

    Ok(())
}
