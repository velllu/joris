use std::net::TcpStream;

use tungstenite::{stream::MaybeTlsStream, WebSocket};

pub type Socket = WebSocket<MaybeTlsStream<TcpStream>>;
