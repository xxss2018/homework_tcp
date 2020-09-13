// 引入std中net下的tcp相关的两个包用于监听端口和处理信息流，引入std下io中的读写两个包，引入线程包

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8899").expect("unable to socket");
//启动tcp监听器，监听8899端口，并将ok情况下的返回值赋予listener；
    
    println!("Listening on port {}", 8899);

//进入for循环,遍历listener.incoming,如果是ok返回，则启动一个线程执行handle_client
    for connection in listener.incoming(){ 
        match connection{
            Ok(stream) => {
	        thread::spawn(|| { handle_client(stream);});
            }
            Err(_) =>{     
                continue;
            },
        }
    }
}


fn handle_client(mut stream: TcpStream){
    println!("connection accepted");

    let mut buffer:[u8;1024] = [0;1024]; //定义一个缓冲区大小为1024

    loop{  //读取数据并回写数据
        match stream.read(&mut buffer){
            Ok(n) => {
                if n == 0 {
                    break;
                }
                if let Err(_) = stream.write(&buffer[..n]){
                    break;
                }
            }
            Err(_) => {
                break;
            }
        }
    }
    println!("disconnected")
}
