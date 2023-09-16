use crate::socks::structs::{AuthMethod, HandShakeRequest};
use crate::socks::utils::check_method_length;

pub fn parse(data: &Vec<u8>) -> Result<HandShakeRequest, String> {
    // 检查数据大小
    if data.len() <= 3 {
        return Err(format!("数据长度错误:{}", data.len()));
    }
    let version = data[0];
    check_method_length(&data[1])?;
    let methods = data[2..].to_vec();
    Ok(HandShakeRequest { version, methods })
}

pub fn build_response(auth_method: AuthMethod) -> Vec<u8> {
    let version = 0x05u8;
    let method = match auth_method {
        AuthMethod::NoAuth => 0x00u8,
        AuthMethod::GSSAPI => 0x01u8,
        AuthMethod::UsernamePassword => 0x02u8,
        AuthMethod::IANAAssigned => 0x03u8,
        AuthMethod::Reserved => 0x80u8,
        AuthMethod::NoAcceptableMethods => 0xFFu8,
    };
    vec![version, method]
}