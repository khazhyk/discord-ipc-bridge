

use std::io::{Result};
use std::env;
pub use std::os::unix::net::UnixStream as RawConn;
use discord_ipc;

pub const DOMAIN_SOCKET_NAME: &str = "discord-ipc-0";

impl discord_ipc::Connectable<RawConn> for RawConn {
    fn raw_connect() -> Result<RawConn> {
        let tmpdir = match env::var("TMPDIR") {
            Ok(t) => t,
            Err(_) => match env::var("TMP") {
                Ok(t) => t,
                Err(_) => "/tmp".to_string()
            }
        };
        println!("{}{}", &tmpdir, DOMAIN_SOCKET_NAME);
        RawConn::connect(tmpdir + DOMAIN_SOCKET_NAME)
    }
}

#[cfg(target_os="macos")] pub use macos::*;
#[cfg(target_os="linux")] pub use linux::*;