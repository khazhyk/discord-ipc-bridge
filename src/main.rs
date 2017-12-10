#![feature(const_size_of)]

mod discord_rpc;
mod windows;
mod ws_server;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate libc;
extern crate serde_json;
extern crate bincode;
extern crate rand;
extern crate time;

use discord_rpc::*;



fn main() {
    let mut connection = windows::connect().unwrap();

    write_frame(
        &mut connection,
        serde_json::to_string(&Handshake {
            v: 1,
            client_id: String::from("387837135568502785"),
        }).unwrap(),
        Opcode::Handshake,
    );

    let time = time::now().to_timespec().sec;

    loop {
        discord_rpc::write_frame(
            &mut connection,
            serde_json::to_string(&Presence {
                nonce: rand::random::<u64>().to_string(),
                cmd: String::from("SET_ACTIVITY"),
                args: PresenceArgs {
                    pid: windows::pid_by_name("chrome.exe").unwrap() as i32,
                    activity: Activity {
                        state: Some(String::from("looking at memes")),
                        details: Some(String::from("110% memes")),
                        timestamps: Some(Timestamps {
                            start: Some(time),
                            end: None,
                        }),
                        assets: Some(Assets {
                            large_image: Some(String::from("ayano-14")),
                            large_text: None,
                            small_image: None,
                            small_text: None,
                        }),
                        party: None,
                        secrets: None,
                        instance: false,
                    },
                },
            }).unwrap(),
            Opcode::Frame,
        );
        std::thread::sleep(std::time::Duration::new(10, 0));
    }
}
