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
        println!("接收到连接: {:?}", addr);
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
    // 解析数据
    let version = &buf[0];
    if *version != 0x05u8 {
        // 不是socks5协议
        println!("不是socks5协议");
        return Err("不是socks5协议".to_string());
    }
    // 获取认证方法数量
    let method_count = &buf[1];
    // 获取认证方法
    let method = &buf[2..2 + *method_count as usize];
    // 将认证方法使用大端序转换为数字
    let method = u16::from_be_bytes(method.try_into().expect("转换失败"));
    println!("认证方法字节数: {}，认证方法: {:?}", method_count, method);
    // 返回握手成功
    let response = [0x05, 0x00];
    stream.write(&response).await.expect("写入数据失败");
    stream.flush().await.expect("刷新数据失败");
    println!("发送数据: {:x?}", &response);
    Ok(Connection { buf, stream })
}

async fn forward(connection: Connection) -> Result<Connection, String> {
    let Connection { mut buf, mut stream } = connection;
    // 读取请求数据
    let data_length = stream.read(&mut buf).await.map_err(|err| err.to_string())?;
    println!("读取数据: {:x?}", &buf[..data_length]);
    Ok(Connection { buf, stream })
}
