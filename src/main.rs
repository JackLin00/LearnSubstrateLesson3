// 导入必要的library
use std::thread;
use std::net::{TcpStream,TcpListener,Shutdown};
use std::io::{Write,Read};


fn handle_client(mut stream: TcpStream) {
    //新建一个具有50个长度的Buffer
    let mut data = [0 as u8; 50]; 
    //不断接收来自客户端的数据
    while match stream.read(&mut data) {
        //若成功接收到数据则将数据原路返回，并且打印
        Ok(size) => {
            let recv_text = String::from_utf8_lossy(&data[0..size]);
            println!("Get data is {},and send back", recv_text);
            stream.write(&data[0..size]).unwrap();
            //清空数据
            for i in &mut data[0..50]{
                *i = 0;
            }
            true
        }
        //失败则释放资源并且打印错误信息
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    }{}
}


#[warn(non_snake_case)]

fn main() {
    // 新建TCP Server并将端口绑定在13145上
    let listener = TcpListener::bind("0.0.0.0:13145").unwrap();
    // 等待TCP Client的连接
    println!("Server listening on port 13145");
    for stream in listener.incoming(){
        match stream{
            //成功则新建线程接收句柄
            Ok(stream) =>{
                println!("New connection {}",stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_client(stream)
                });
            }
            //失败则打印错误信息
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    //释放资源
    drop(listener);
}
