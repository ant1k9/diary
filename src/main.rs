use diary::config::*;
use diary::date::*;
use diary::db::*;
use diary::errors::*;

use prettytable::{Cell, Row, Table, format};
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
    date: String,
    activity: String,
}

#[derive(Debug, StructOpt)]
pub struct Show {
    activity: String,
    #[structopt(short, long)]
    first: Option<u32>,
    #[structopt(short, long)]
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
    check_date_format(&date)?;

    if let Some(activity_config) = config.iter().find(|record| record.name == activity) {
        ensure_table_is_ready(activity_config)?;
        let answers = activity_config
            .fields
            .iter()
            .map(|f| {
                println!("{}", f.title);
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                input.to_owned()
            })
            .collect::<Vec<_>>();
        return save_diary_record(activity_config, answers, date);
    }

    println!("no such activity {}", activity);
    Ok(())
}

fn edit_diary_record(config: &Config, activity: &str, date: String) -> BoxedErrorResult<()> {
    add_diary_record(config, activity, Some(date))
}

fn show_diary_record(
    config: &Config,
    activity: &str,
    first: Option<u32>,
    last: Option<u32>,
    date: Option<String>,
) -> BoxedErrorResult<()> {
    if first.is_some() && last.is_some() {
        return Err(String::from("provide only first or last argument, not both").into());
    }
    check_date_format(&date)?;

    if let Some(activity_config) = config.iter().find(|record| record.name == activity) {
        ensure_table_is_ready(activity_config)?;
        let diary_records: Vec<Vec<String>> = match first {
            Some(value) => get_diary_records(activity_config, value, Sorting::ASC, date).unwrap(),
            None => match last {
                Some(value) => {
                    get_diary_records(activity_config, value, Sorting::DESC, date).unwrap()
                }
                None => get_diary_records(activity_config, 1, Sorting::DESC, date).unwrap(),
            },
        };

        let mut header = vec![Cell::new("date")];
        activity_config
            .fields
            .iter()
            .for_each(|f| header.push(Cell::new(&f.name)));

        let mut table = Table::new();
        table.set_titles(Row::new(header));
        diary_records.iter().for_each(|record| {
            table.add_row(Row::new(
                record.iter().map(|f| Cell::new(f)).collect::<Vec<_>>(),
            ));
        });
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.printstd();
    }

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
        Command::Edit(edit) => edit_diary_record(&config, &edit.activity, edit.date)?,
        Command::Show(show) => {
            show_diary_record(&config, &show.activity, show.first, show.last, show.date)?
        }
    }

    Ok(())
}
