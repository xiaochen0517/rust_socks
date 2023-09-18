use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::socks::structs::{Connection, SocksAddressType, SocksCMD};
use crate::socks::utils::check_version;
use crate::socks::parser::forward;

pub async fn forward(mut connection: Connection) -> Result<(), String> {
    // 读取请求数据
    let data_vec = connection.read().await?;
    let forward_request = forward::parse(&data_vec)?;
    check_version(&(forward_request.version))?;
    // 与目标服务器建立连接
    println!("{}", forward_request.address);
    let mut des_stream = tokio::net::TcpStream::connect(&forward_request.address).await
        .map_err(|err| err.to_string())?;
    // 响应客户端
    let response = forward::build_response(&forward_request, 0x00);
    connection.write(&response).await?;
    // 转发数据
    tokio::io::copy_bidirectional(&mut connection.stream, &mut des_stream).await
        .map_err(|err| err.to_string())?;
    println!("转发数据完成");
    Ok(())
}