use tokio::net::TcpStream;
use crate::socks::forward::forward;
use crate::socks::handshake::handshake;
use crate::socks::structs::Connection;

mod structs;
mod handshake;
mod utils;
mod forward;

pub async fn handle_connection(stream: TcpStream) -> Result<(), String> {
    // 处理连接
    let buf = [0u8; 1024];
    let connection = Connection { buf, stream };
    let connection = handshake(connection).await?;
    let _connection = forward(connection).await?;
    Ok(())
}