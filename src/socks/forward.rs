use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::socks::structs::{Connection, SocksAddressType, SocksCMD};
use crate::socks::utils::check_version;

pub async fn forward(connection: Connection) -> Result<Connection, String> {
    let Connection { mut buf, mut stream } = connection;
    // 读取请求数据
    let data_length = stream.read(&mut buf).await.map_err(|err| err.to_string())?;
    println!("读取数据: {:x?}", &buf[..data_length]);
    check_version(&buf[0])?;
    let _socks_cmd = parse_socks_cmd(&buf[1])?;
    let socks_address_type = parse_socks_address_type(&buf[3])?;
    let adderss_str = parse_socks_address_port(socks_address_type, &buf[4..])?;
    // 与目标服务器建立连接
    println!("连接目标服务器: {}...", adderss_str);
    let mut des_stream = tokio::net::TcpStream::connect(adderss_str).await
        .map_err(|err| err.to_string())?;
    // 响应客户端
    let response = [0x05, 0x00, 0x00, 0x01, buf[4], buf[5], buf[6], buf[7], buf[8], buf[9]];
    stream.write(&response).await.map_err(|err| err.to_string())?;
    stream.flush().await.map_err(|err| err.to_string())?;
    println!("发送数据: {:x?}", &response);
    // 转发数据
    tokio::io::copy_bidirectional(&mut stream, &mut des_stream).await
        .map_err(|err| err.to_string())?;
    println!("转发数据完成");
    Ok(Connection { buf, stream })
}

fn parse_socks_cmd(cmd: &u8) -> Result<SocksCMD, String> {
    return match *cmd {
        0x01u8 => {
            Ok(SocksCMD::Connect)
        }
        0x02u8 => {
            Ok(SocksCMD::Bind)
        }
        0x03u8 => {
            Ok(SocksCMD::UdpAssociate)
        }
        _ => {
            Err("不支持的命令".to_string())
        }
    };
}

fn parse_socks_address_type(types: &u8) -> Result<SocksAddressType, String> {
    return match *types {
        0x01u8 => {
            Ok(SocksAddressType::IPv4)
        }
        0x03u8 => {
            Ok(SocksAddressType::Domain)
        }
        0x04u8 => {
            Ok(SocksAddressType::IPv6)
        }
        _ => {
            Err("不支持的地址类型".to_string())
        }
    };
}

fn parse_socks_address_port(types: SocksAddressType, data_arrays: &[u8]) -> Result<String, String> {
    match types {
        SocksAddressType::IPv4 => {
            let ip_str = format!("{}.{}.{}.{}", data_arrays[0], data_arrays[1], data_arrays[2], data_arrays[3]);
            let port_str = format!("{}", u16::from_be_bytes([data_arrays[4], data_arrays[5]]));
            Ok(format!("{}:{}", ip_str, port_str))
        }
        SocksAddressType::Domain => { Err("不支持的地址类型".to_string()) }
        SocksAddressType::IPv6 => { Err("不支持的地址类型".to_string()) }
    }
}