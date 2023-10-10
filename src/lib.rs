use sockets::Socket;

pub mod errors;
pub mod login;
pub mod sockets;

const URL_BASE: &str = "https://discord.com/api/v9";
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/117.0";

pub struct Joris {
    pub token: String,
    socket: Socket,
}
