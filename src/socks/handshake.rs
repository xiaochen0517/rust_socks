use crate::socks::parser::hand_shake;
use crate::socks::structs::{AuthMethod, Connection};
use crate::socks::utils::check_version;

pub async fn handshake(mut connection: Connection) -> Result<Connection, String> {
    let read_data_vec = connection.read().await?;
    println!("读取数据: {:x?}", read_data_vec);
    let hand_shake_request = hand_shake::parse(&read_data_vec)?;
    check_version(&hand_shake_request.version)?;
    let response = hand_shake::build_response(AuthMethod::NoAuth);
    connection.write(&response).await?;
    println!("发送数据: {:x?}", &response);
    Ok(connection)
}