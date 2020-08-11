use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    // 绑定ip 端口
    let l = TcpListener::bind("127.0.0.1:9999");
    // 绑定结果处理
    let listener = match l {
        Ok(l) => l,
        Err(e) => panic!("bind socket error: {:?}", e),
    };
    // 接收连接
    for stream in listener.incoming() {
        // 获得 tcp流
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    // 接收数据到buffer
    stream.read(&mut buffer).unwrap();
    // 转成字符串
    let response = String::from_utf8_lossy(&buffer[..]);
    // 发送字节流
    stream.write(response.as_bytes()).unwrap();
    // 清理流数据，发送出去
    stream.flush().unwrap();
}
