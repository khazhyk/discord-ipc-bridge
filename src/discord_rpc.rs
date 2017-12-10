pub const MAX_RPC_FRAME_SIZE: usize = 64 * 1024;
pub const MAX_RPC_CONTENT_SIZE: usize = MAX_RPC_FRAME_SIZE -
    ::std::mem::size_of::<MessageFrameHeader>();


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

#[derive(Serialize, Deserialize)]
pub struct Presence {
    pub nonce: String,
    pub cmd: String,
    pub args: PresenceArgs,
}

#[derive(Serialize, Deserialize)]
pub struct PresenceArgs {
    pub pid: i32,
    pub activity: Activity,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Timestamps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Party {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Vec<i32>>,
}

#[derive(Serialize, Deserialize)]
pub struct Secrets {
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    pub _match: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spectate: Option<String>,
}

pub fn write_frame<S: Into<String>>(stream: &mut (::std::io::Write), content: S, opcode: Opcode) {
    let content = content.into();
    println!("{}", &content);
    let content_bytes = content.as_bytes();
    let header = MessageFrameHeader {
        opcode: opcode,
        length: content_bytes.len() as u32,
    };
    stream
        .write_all(
            ::bincode::serialize(
                &header,
                ::bincode::Bounded(::std::mem::size_of::<MessageFrameHeader>() as u64),
            ).unwrap()
                .as_slice(),
        )
        .unwrap();
    // TODO : check content length
    stream.write_all(content_bytes).unwrap();
    stream.flush().unwrap();
}
