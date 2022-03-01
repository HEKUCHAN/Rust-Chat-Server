use std::io::{stdin, Write, BufRead, BufReader};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    // サーバーのアドレスを指定
    let server_addr = "127.0.0.1:8888";
    let mut socket = TcpStream::connect(server_addr)
        .expect("サーバーと接続できません。");
    socket.set_nonblocking(true).expect("利用不可");
    println!("{}と接続しました。", server_addr);
    // 受信用のスレッドを開始
    start_thread(socket.try_clone().unwrap());
    // 標準入力からユーザー名を得る
    let user = input("お名前は？");
    println!("{}さん、メッセージを入力してください。", user);
    loop {
        let msg = input("");
        let msg = format!("{}> {}\n", user, msg);
        let buf = msg.as_bytes();
        socket.write_all(buf).unwrap();
    }
}

// スレッドを開始してサーバーからメッセージを受信する
fn start_thread(socket: TcpStream) {
    let mut reader = BufReader::new(socket);
    thread::spawn(move || loop {
        // サーバーからメッセージを受信
        let mut buf = String::new();
        if let Ok(n) = reader.read_line(&mut buf) {
            if n > 0 { // 受信内容を画面に表示
                println!("[受信] {}", buf.trim());
            }
        }
        thread::sleep(Duration::from_millis(100));
    });
}

fn input(msg: &str) -> String {
    if msg != "" { println!("{}", msg); }
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("標準入力エラー");
    String::from(buf.trim())
}
