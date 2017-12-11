pub const MAX_RPC_FRAME_SIZE: usize = 64 * 1024;
pub const MAX_RPC_CONTENT_SIZE: usize = MAX_RPC_FRAME_SIZE -
    ::std::mem::size_of::<MessageFrameHeader>();

use serde_json;
use std::io::Result as IOResult;
use std::io::{Error, ErrorKind};

#[derive(Serialize, Deserialize)]
#[repr(u32)]
pub enum Opcode {
    Handshake,
    Frame,
    Close,
    Ping,
    Pong,
}

#[derive(Serialize, Deserialize)]
pub struct MessageFrameHeader {
    pub opcode: Opcode,
    pub length: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Handshake {
    pub v: i32,
    pub client_id: String,
}

#[derive(Serialize, Deserialize, Default, DefaultBuilder)]
pub struct Presence {
    pub nonce: String,
    pub cmd: String,
    pub args: PresenceArgs,
}

#[derive(Serialize, Deserialize, Default, DefaultBuilder)]
pub struct PresenceArgs {
    pub pid: i32,
    pub activity: Activity,
}

#[derive(Serialize, Deserialize, Default, DefaultBuilder)]
pub struct Activity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<Timestamps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Assets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party: Option<Party>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Secrets>,
    pub instance: bool,
}

#[derive(Serialize, Deserialize, Default, DefaultBuilder)]
pub struct Timestamps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
}

#[derive(Serialize, Deserialize, Default, DefaultBuilder)]
pub struct Assets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_text: Option<String>,
}

#[derive(Serialize, Deserialize, Default, DefaultBuilder)]
pub struct Party {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Vec<i32>>,
}

#[derive(Serialize, Deserialize, Default, DefaultBuilder)]
pub struct Secrets {
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    pub _match: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spectate: Option<String>,
}

pub trait Connectable<C> {
    fn raw_connect() -> IOResult<C>;
}

pub trait RawIpcConnection<T>
    : ::std::io::Read + ::std::io::Write + Connectable<T> {
    fn write_frame<S: Into<String>>(&mut self, content: S, opcode: Opcode)
        -> ::std::io::Result<()>;
    fn ipc_connect<S: Into<String>>(client_id: S) -> IOResult<T>;
}

impl<T> RawIpcConnection<T> for T
where
    T: ::std::io::Read + ::std::io::Write + Connectable<T>,
{
    fn write_frame<S: Into<String>>(
        &mut self,
        content: S,
        opcode: Opcode,
    ) -> ::std::io::Result<()> {
        let content = content.into();
        println!("{}", &content);
        let content_bytes = content.as_bytes();
        if content_bytes.len() > MAX_RPC_CONTENT_SIZE {
            return Err(Error::new(ErrorKind::Other, "Message too large"));
        }
        let header = MessageFrameHeader {
            opcode: opcode,
            length: content_bytes.len() as u32,
        };
        try!(
            self.write_all(
                ::bincode::serialize(
                    &header,
                    ::bincode::Bounded(::std::mem::size_of::<MessageFrameHeader>() as u64),
                ).unwrap()
                    .as_slice(),
            )
        );
        try!(self.write_all(content_bytes));
        try!(self.flush());
        Ok(())
    }

    fn ipc_connect<S: Into<String>>(client_id: S) -> IOResult<T> {
        let mut conn = try!(Self::raw_connect());
        let client_id = client_id.into();
        try!(
            conn.write_frame(
                serde_json::to_string(&Handshake {
                    v: 1,
                    client_id: client_id.clone(),
                }).unwrap(),
                Opcode::Handshake,
            )
        );

        Ok(conn)
    }
}
