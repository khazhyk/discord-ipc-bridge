#![feature(const_size_of)]

mod discord_ipc;
mod ws_server;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate libc;
extern crate serde_json;
extern crate bincode;
extern crate rand;
extern crate time;

#[macro_use]
extern crate builder;

use discord_ipc::*;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
use windows as util;


fn main() {
    let mut connection = util::RawConn::ipc_connect("387837135568502785").unwrap();

    let time = time::now().to_timespec().sec;

    loop {
        let memes = serde_json::to_string(&Presence::builder()
            .nonce(rand::random::<u64>().to_string())
            .cmd("SET_ACTIVITY")
            .args(
                PresenceArgs::builder()
                    .pid(util::pid_by_name("chrome.exe").unwrap() as i32)
                    .activity(
                        Activity::builder()
                            .state("looking at memes".to_string())
                            .details("110% memes".to_string())
                            .timestamps(Timestamps::builder().start(time))
                            .assets(
                                Assets::builder()
                                    .large_image("ayano-14".to_string())
                                    .large_text("110% memes".to_string()),
                            ),
                    ),
            )
            .build()).unwrap();

        connection.write_frame(memes, Opcode::Frame).unwrap();
        std::thread::sleep(std::time::Duration::new(10, 0));
    }
}
