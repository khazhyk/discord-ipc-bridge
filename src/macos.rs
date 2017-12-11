use std::io::{Error, ErrorKind, Result};

use regex::Regex;

pub const CHROME_NAME : &str = "Google Chrome.app";

pub fn pid_by_name<S: Into<String>>(name_query: S) -> Result<i32> {
    let name_query = name_query.into();

    let ps_res = ::std::process::Command::new("ps").arg("aux").output().unwrap();
    let lines = unsafe {String::from_utf8_unchecked(ps_res.stdout)};
    
    for ref line in lines.split("\n") {
        if line.contains(&name_query) {
            lazy_static! {
                static ref PID_MATCH: Regex = Regex::new(r"\w+\s+(\d+).*").unwrap();
            }
            let captures = PID_MATCH.captures(line).unwrap();
            return Ok(captures[1].parse::<i32>().unwrap());
        }
    }
    return Err(Error::new(ErrorKind::Other, "Not found"));
}