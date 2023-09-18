use tokio::net::TcpListener;
use crate::socks::handle_connection;

mod socks;

const LISTENER_ADDR: &str = "127.0.0.1:8888";

pub async fn run() {
    // 监听端口
    let listener = TcpListener::bind(LISTENER_ADDR).await.expect("端口监听失败");
    println!("启动监听: {}", format!("socks5://{}/", LISTENER_ADDR));
    // 循环监听
    loop {
        let (stream, addr) = listener.accept().await.expect("连接失败");
        print!("{:?} --> ", addr);
        // 使用tokio创建一个新的任务
        tokio::spawn(async move {
            // 处理连接
            handle_connection(stream).await.unwrap_or_else(|err| {
                println!("\n处理连接失败: {}", err);
            });
        });
    }
}