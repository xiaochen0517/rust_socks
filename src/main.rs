use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const LISTENER_ADDR: &str = "127.0.0.1:8888";

#[tokio::main]
async fn main() {
    // 监听端口
    let listener = TcpListener::bind(LISTENER_ADDR).await.expect("端口监听失败");
    println!("启动监听: {}", format!("socks5://{}/", LISTENER_ADDR));
    // 循环监听
    loop {
        let (stream, addr) = listener.accept().await.expect("连接失败");
        println!("接收到请求: {:?}", addr);
        // 使用tokio创建一个新的任务
        tokio::spawn(async move {
            // 处理连接
            handle_connection(stream).await.unwrap_or_else(|err| {
                println!("处理连接失败: {}", err);
            });
        });
    }
}

struct Connection {
    buf: [u8; 1024],
    stream: TcpStream,
}

async fn handle_connection(stream: TcpStream) -> Result<(), String> {
    // 处理连接
    let buf = [0u8; 1024];
    let connection = Connection { buf, stream };
    let connection = handshake(connection).await?;
    let _connection = forward(connection).await?;
    Ok(())
}

async fn handshake(connection: Connection) -> Result<Connection, String> {
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

fn check_method_length(length: &u8) -> Result<usize, String> {
    if *length <= 0 {
        return Err("方法数长度字节数不能小于0".to_string());
    }
    Ok(*length as usize)
}

async fn forward(connection: Connection) -> Result<Connection, String> {
    let Connection { mut buf, mut stream } = connection;
    // 读取请求数据
    let data_length = stream.read(&mut buf).await.map_err(|err| err.to_string())?;
    println!("读取数据: {:x?}", &buf[..data_length]);
    check_version(&buf[0])?;
    let _socks_cmd = parse_socks_cmd(&buf[1])?;
    let socks_address_type = parse_socks_address_type(&buf[3])?;
    parse_socks_address(socks_address_type, &buf[4..])?;
    Ok(Connection { buf, stream })
}

#[derive(Debug)]
enum SocksCMD {
    Connect,
    Bind,
    UdpAssociate,
}

#[derive(Debug)]
enum SocksAddressType {
    IPv4,
    Domain,
    IPv6,
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

fn parse_socks_address(types: SocksAddressType, data_arrays: &[u8]) -> Result<(), String> {
    println!("解析地址类型: {:?}", types);
    match types {
        SocksAddressType::IPv4 => println!("解析IPv4地址: {:?}", data_arrays[..4].to_vec()),
        SocksAddressType::Domain => {}
        SocksAddressType::IPv6 => {}
    }

    Ok(())
}

fn check_version(version: &u8) -> Result<(), String> {
    if *version != 0x05u8 {
        return Err("不是socks5协议".to_string());
    }
    Ok(())
}
