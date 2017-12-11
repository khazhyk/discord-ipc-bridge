use std::io::{Error, ErrorKind, Result};
extern crate psutil;

pub fn pid_by_name<S: Into<String>>(name_query: S) -> Result<i32> {
    let procs = try!(psutil::process::all());
    let name_query = name_query.into();

    for ref pro in procs {
        if pro.comm == name_query {
            return Ok(pro.pid);
        }
    }

    return Err(Error::new(ErrorKind::Other, "Not found"));
}