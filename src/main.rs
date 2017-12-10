#![feature(const_size_of)]
const MAX_RPC_FRAME_SIZE : usize = 64 * 1024;
const MAX_RPC_CONTENT_SIZE : usize = MAX_RPC_FRAME_SIZE - std::mem::size_of::<MessageFrameHeader>();

const WINDOWS_PIPE_ADDR : &str = "//./pipe/discord-ipc-0";

use std::io::Read;
use std::io::Write;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate libc;
extern crate serde_json;
extern crate bincode;
extern crate windows_named_pipe;
extern crate rand;
extern crate time;

use windows_named_pipe::PipeStream;

#[derive(Serialize, Deserialize)]
#[repr(u32)]
enum Opcode {
	Handshake,
	Frame,
	Close,
	Ping,
	Pong
}

#[derive(Serialize, Deserialize)]
#[repr(C)]
struct MessageFrameHeader {
	opcode: Opcode,
	length: u32
}

#[derive(Serialize, Deserialize)]
struct Handshake {
	v: i32,
	client_id: String
}

#[derive(Serialize, Deserialize)]
struct Presence {
	nonce: String,
	cmd: String,
	args: PresenceArgs
}

#[derive(Serialize, Deserialize)]
struct PresenceArgs {
	pid: i32,
	activity: Activity
}

#[derive(Serialize, Deserialize)]
struct Activity {
	#[serde(skip_serializing_if="Option::is_none")]
	state: Option<String>,
	#[serde(skip_serializing_if="Option::is_none")]
	details: Option<String>,
	#[serde(skip_serializing_if="Option::is_none")]
	timestamps: Option<Timestamps>,
	#[serde(skip_serializing_if="Option::is_none")]
	assets: Option<Assets>,
	#[serde(skip_serializing_if="Option::is_none")]
	party: Option<Party>,
	#[serde(skip_serializing_if="Option::is_none")]
	secrets: Option<Secrets>,
	instance: bool
}

#[derive(Serialize, Deserialize)]
struct Timestamps {
	#[serde(skip_serializing_if="Option::is_none")]
	start: Option<i64>,
	#[serde(skip_serializing_if="Option::is_none")]
	end: Option<i64>
}

#[derive(Serialize, Deserialize)]
struct Assets {
	#[serde(skip_serializing_if="Option::is_none")]
	large_image: Option<String>,
	#[serde(skip_serializing_if="Option::is_none")]
	large_text: Option<String>,
	#[serde(skip_serializing_if="Option::is_none")]
	small_image: Option<String>,
	#[serde(skip_serializing_if="Option::is_none")]
	small_text: Option<String>
}

#[derive(Serialize, Deserialize)]
struct Party {
	#[serde(skip_serializing_if="Option::is_none")]
	id: Option<String>,
	#[serde(skip_serializing_if="Option::is_none")]
	size: Option<Vec<i32>>
}

#[derive(Serialize, Deserialize)]
struct Secrets {
	#[serde(rename="match", skip_serializing_if="Option::is_none")]
	_match: Option<String>,
	#[serde(skip_serializing_if="Option::is_none")]
	join: Option<String>,
	#[serde(skip_serializing_if="Option::is_none")]
	spectate: Option<String>
}

fn write_frame<S: Into<String>>(stream: &mut PipeStream, content: S, opcode: Opcode) {
	let content = content.into();
	println!("{}", &content);
	let content_bytes = content.as_bytes();
	let header = MessageFrameHeader {
		opcode: opcode,
		length: content_bytes.len() as u32
	};
	stream.write_all(bincode::serialize(&header, bincode::Bounded(std::mem::size_of::<MessageFrameHeader>() as u64)).unwrap().as_slice()).unwrap();
	stream.write_all(content_bytes).unwrap();
	stream.flush().unwrap();
}

fn main() {
	let mut named_pipe = PipeStream::connect(WINDOWS_PIPE_ADDR).unwrap();
	write_frame(&mut named_pipe, serde_json::to_string(&Handshake {
		v: 1,
		client_id: String::from("387837135568502785")
	}).unwrap(), Opcode::Handshake);

	let time = time::now().to_timespec().sec;

	loop {
		write_frame(&mut named_pipe, serde_json::to_string(&Presence {
			nonce: rand::random::<u64>().to_string(),
			cmd: String::from("SET_ACTIVITY"),
			args: PresenceArgs {
				pid: unsafe {libc::getpid()},
				activity: Activity {
					state: Some(String::from("looking at memes")),
					details: Some(String::from("110% memes")),
					timestamps: Some(Timestamps {
						start: Some(time),
						end: None
					}),
					assets: Some(Assets {
						large_image: Some(String::from("ayano-14")),
						large_text: None,
						small_image: None,
						small_text: None
					}),
					party: None,
					secrets: None,
					instance: false
				}
			}
		}).unwrap(), Opcode::Frame);

		std::thread::sleep(std::time::Duration::new(10, 0));
	}

	// let mut response = String::new();
	// named_pipe.read_to_string(&mut response);
	// println!("Hello world! {}", response);
}
