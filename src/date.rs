use crate::errors::BoxedErrorResult;
use regex::Regex;

pub fn check_date_format(date: &Option<String>) -> BoxedErrorResult<()> {
    if let Some(ref d) = date {
        let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").expect("cannot compile regexp to check date");
        if !re.is_match(d) {
            return Err(String::from("invalid formar of the date, expected 2000-01-01").into());
        }
    }
    Ok(())
}
