use crate::socks::structs::{ForwardRequest, SocksAddressType};
use crate::socks::structs::{get_socks_address_type, get_socks_cmd};

pub fn parse(data: &Vec<u8>) -> Result<ForwardRequest, String> {
    if data.len() < 7 {
        return Err(format!("数据长度错误:{}", data.len()));
    }
    let version = data[0];
    let command = get_socks_cmd(&data[1])?;
    let _reserved = data[2];
    let address_type = get_socks_address_type(&data[3])?;
    let host = data[4..data.len() - 2].to_vec();
    let port = data[data.len() - 2..].to_vec();
    let address = format!("{}.{}.{}.{}:{}", host[0], host[1], host[2], host[3],
                          u16::from_be_bytes([port[0], port[1]]));
    Ok(ForwardRequest { version, cmd: command, address_type, host, port, address })
}

pub fn build_response(request: &ForwardRequest, status: u8) -> Vec<u8> {
    let mut response_vec: Vec<u8> = Vec::new();
    response_vec.push(0x05); // version
    response_vec.push(status); // status
    response_vec.push(0x00); // reserved
    match request.address_type {
        SocksAddressType::IPv4 => {
            response_vec.push(0x01); // address type
            response_vec.extend_from_slice(&request.host); // address
            response_vec.extend_from_slice(&request.port); // port
        }
        SocksAddressType::Domain => {}
        SocksAddressType::IPv6 => {}
    }
    response_vec
}