use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::socks::structs::Connection;
use crate::socks::utils::{check_method_length, check_version};

pub async fn handshake(connection: Connection) -> Result<Connection, String> {
    let Connection { mut buf, mut stream } = connection;
    let data_length = stream.read(&mut buf).await.map_err(|err| err.to_string())?;
    println!("读取数据: {:x?}", &buf[..data_length]);
    check_version(&buf[0])?;
    let _method_length = check_method_length(&buf[1])?;
    let response = [0x05, 0x00];
    stream.write(&response).await.map_err(|err| err.to_string())?;
    stream.flush().await.map_err(|err| err.to_string())?;
    println!("发送数据: {:x?}", &response);
    Ok(Connection { buf, stream })
}