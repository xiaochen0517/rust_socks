use std::cell::{RefCell, RefMut};
use std::error::Error;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

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
            handle_connection(stream).await;
        });
    }
}

async fn handle_connection(stream: TcpStream) {
    // 处理连接
    let ref_cell_buf = Arc::new(Mutex::new([0u8; 1024]));
    let ref_cell_stream = Arc::new(Mutex::new(stream));
    match handshake(&ref_cell_buf, &ref_cell_stream).await {
        Ok(_) => println!("握手成功"),
        Err(e) => {
            println!("握手失败: {}", e);
            return;
        }
    }
    match forward(&ref_cell_buf, &ref_cell_stream).await {
        Ok(_) => println!("转发成功"),
        Err(e) => {
            println!("转发失败: {}", e);
            return;
        }
    }
}

async fn handshake(ref_cell_buf: &Arc<Mutex<[u8; 1024]>>, ref_cell_stream: &Arc<Mutex<TcpStream>>) -> Result<(), String> {
    let mut buf_ref_mut = ref_cell_buf.lock().await;
    let mut stream_ref_mut = ref_cell_stream.lock().await;
    let data_length = stream_ref_mut.read(&mut *buf_ref_mut).await.expect("读取数据失败");
    println!("读取数据: {:x?}", &buf_ref_mut[..data_length]);
    // 解析数据
    let version = &buf_ref_mut[0];
    if *version != 0x05u8 {
        // 不是socks5协议
        println!("不是socks5协议");
        return Err("不是socks5协议".to_string());
    }
    // 获取认证方法数量
    let method_count = &buf_ref_mut[1];
    // 获取认证方法
    let method = &buf_ref_mut[2..2 + *method_count as usize];
    // 将认证方法使用大端序转换为数字
    let method = u16::from_be_bytes(method.try_into().expect("转换失败"));
    println!("认证方法字节数: {}，认证方法: {:?}", method_count, method);
    // 返回握手成功
    let response = [0x05, 0x00];
    stream_ref_mut.write(&response).await.expect("写入数据失败");
    stream_ref_mut.flush().await.expect("刷新数据失败");
    println!("发送数据: {:x?}", &response);
    Ok(())
}

async fn forward(ref_cell_buf: &Arc<Mutex<[u8; 1024]>>, ref_cell_stream: &Arc<Mutex<TcpStream>>) -> Result<(), String> {
    let mut buf_ref_mut = ref_cell_buf.lock().await;
    let mut stream_ref_mut = ref_cell_stream.lock().await;
    // 读取请求数据
    let data_length = stream_ref_mut.read(&mut *buf_ref_mut).await.map_err(|err| err.to_string())?;
    println!("读取数据: {:x?}", &buf_ref_mut[..data_length]);
    Ok(())
}
