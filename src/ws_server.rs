
pub const DISCORD_IPC_BRIDGE_START_PORT: u16 = 6313;

extern crate ws;

use serde_json;
use discord_ipc;
use discord_ipc::*;

use rand;

#[cfg(windows)]
use windows as util;
#[cfg(unix)]
use unix as util;

#[derive(Serialize, Deserialize)]
#[serde(tag = "opcode", content = "cmd")]
pub enum IpcCommand {
    Hand(discord_ipc::Handshake),
    Activity(discord_ipc::Activity),
}

struct IpcBridgeHandler<T>
where
    T: discord_ipc::RawIpcConnection<T>,
{
    out: ws::Sender,
    bridge: T,
}

impl<T> ws::Handler for IpcBridgeHandler<T>
where
    T: discord_ipc::RawIpcConnection<T>,
{
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        if let ws::Message::Text(msg) = msg {
            let cmd: IpcCommand = match serde_json::from_str(&msg) {
                Ok(m) => m,
                Err(err) => {
                    println!("Invalid: {}", msg);
                    return self.out.close_with_reason(
                        ws::CloseCode::Invalid,
                        format!("Failed to parse json message!: {}", err),
                    );
                }
            };

            match cmd {
                IpcCommand::Hand(hand) => {
                    println!("client_id: {}", hand.client_id);
                    try!(self.bridge.handshake(hand.client_id));
                }
                IpcCommand::Activity(presence) => {
                    try!(
                        self.bridge.rich_presence(
                            Presence::builder()
                                .nonce(rand::random::<u64>().to_string())
                                .cmd("SET_ACTIVITY")
                                .args(
                                    PresenceArgs::builder()
                                        .pid(util::pid_by_name(util::CHROME_NAME).unwrap())
                                        .activity(presence),
                                ),
                        )
                    );
                    ::std::thread::sleep(::std::time::Duration::new(4, 0));
                }
            }

            return self.out.send(("lol".to_string() + &msg));
        }
        self.out.close(ws::CloseCode::Unsupported)
    }
}

trait IpcBridgeHandlerFactoryFactory<T>
where
    T: discord_ipc::RawIpcConnection<T>,
{
    fn factory(ws::Sender) -> IpcBridgeHandler<T>;
}



impl<T> IpcBridgeHandlerFactoryFactory<T> for T
where
    T: discord_ipc::RawIpcConnection<T>,
{
    fn factory(out: ws::Sender) -> IpcBridgeHandler<T> {
        IpcBridgeHandler {
            out: out,
            bridge: T::raw_connect().unwrap(),
        }
    }
}

pub fn websocket_thread() {
    ws::listen(
        ("127.0.0.1", DISCORD_IPC_BRIDGE_START_PORT),
        util::RawConn::factory,
    ).unwrap();
}
