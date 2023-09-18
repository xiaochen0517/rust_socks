use tokio::net::TcpStream;
use crate::socks::forward::forward;
use crate::socks::handshake::handshake;
use crate::socks::structs::Connection;

mod structs;
mod handshake;
mod utils;
mod forward;
mod parser;

pub async fn handle_connection(stream: TcpStream) -> Result<(), String> {
    // 处理连接
    let connection = Connection::new(stream);
    let connection = handshake(connection).await?;
    forward(connection).await?;
    Ok(())
}