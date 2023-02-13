use crate::config::Activity;
use crate::errors::BoxedErrorResult;
use regex::Regex;
use rusqlite::{params_from_iter, Connection};

struct TableInfo {
    name: String,
}

pub fn ensure_table_is_ready(activity: &Activity) -> BoxedErrorResult<()> {
    let re = Regex::new(r"[^a-zA-Z\d]").expect("cannot compile regexp to normalize name");
    let table_name = normalize_name(&re, &activity.name);
    let conn = Connection::open("diary.sqlite3")?;

    conn.execute(
        format!("CREATE TABLE IF NOT EXISTS {} (date TEXT UNIQUE DEFAULT (DATE('now')))", table_name).as_str(),
        (),
    )?;

    let mut stmt = conn.prepare(format!("PRAGMA table_info({})", table_name).as_str())?;
    let fields = stmt.query_map([], |row| Ok(TableInfo { name: row.get(1)? }))?;
    let fields = fields.into_iter().map(|f| f.unwrap()).collect::<Vec<_>>();

    activity.fields.iter().for_each(|f| {
        let field_name = normalize_name(&re, &f.name);
        if fields
            .iter()
            .find(|db_field| db_field.name == field_name)
            .is_none()
        {
            conn.execute(
                format!("ALTER TABLE {} ADD COLUMN {} TEXT", table_name, field_name,).as_str(),
                (),
            )
            .unwrap();
        }
    });

    Ok(())
}

pub fn save_diary_record(activity: &Activity, answers: Vec<String>) -> BoxedErrorResult<()> {
    let re = Regex::new(r"[^a-zA-Z\d]").expect("cannot compile regexp to normalize name");
    let table_name = normalize_name(&re, &activity.name);
    let conn = Connection::open("diary.sqlite3")?;

    let mut stmt = format!("INSERT INTO {} (", table_name);
    let mut values_stmt = "".to_owned();
    for i in 0..activity.fields.len() {
        if i == 0 {
            stmt = format!("{}{}", stmt, normalize_name(&re, &activity.fields[i].name));
            values_stmt = "?1".to_owned();
            continue;
        }
        stmt = format!("{},{}", stmt, normalize_name(&re, &activity.fields[i].name));
        values_stmt = format!("{},${}", values_stmt, i + 1);
    }
    stmt = format!("{}) VALUES ({})", stmt, values_stmt);

    conn.execute(&stmt, params_from_iter(answers))?;

    Ok(())
}

fn normalize_name(re: &Regex, name: &str) -> String {
    re.replace_all(name, "_").to_string()
}
