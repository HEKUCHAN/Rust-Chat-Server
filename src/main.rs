use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write};
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    // サーバーのアドレスとポート番号を指定
    let server_addr = "127.0.0.1:8888";
    // スレッド間通信を用意
    let (tx, rx) = mpsc::channel::<String>();
    // クライアント一覧を覚えておく
    let mut clients: Vec<TcpStream> = Vec::new();

    // サーバーを起動
    let server = TcpListener::bind(server_addr)
        .expect("サーバーの起動に失敗");
    server.set_nonblocking(true).expect("利用不可");
    println!("{}でサーバーを起動しました。", server_addr);

    loop {
        // クライアントの待ち受け
        if let Ok((client, addr)) = server.accept() {
            println!("クライアントが接続: {}", addr);
            clients.push(client.try_clone().unwrap());
            start_thread(client, tx.clone());
        }

        // スレッド間通信の待ち受け
        if let Ok(msg) = rx.try_recv() {
            println!("全員に送信: {}", msg.trim());
            clients = send_all(clients, &msg);
        }
        thread::sleep(Duration::from_millis(100));
    }
}

fn start_thread(client: TcpStream, tx: mpsc::Sender<String>) {
    let mut reader = BufReader::new(client);
    thread::spawn(move || loop {
        // メッセージを待つ
        let mut msg = String::new();
        if let Ok(n) = reader.read_line(&mut msg) {
            // 受信内容をメインスレッドに返信
            if n > 0 { tx.send(msg).unwrap(); }
        }
        thread::sleep(Duration::from_millis(100));
    });
}

fn send_all(clients: Vec<TcpStream>, s: &str) -> Vec<TcpStream> {
    let mut collector = vec![];
    for mut socket in clients.into_iter() {
        // 文字列をバイト列に変換して送信
        let bytes = String::from(s).into_bytes();
        if let Err(e) = socket.write_all(&bytes) {
            println!("送信エラー: {}", e);
            continue;
        }
        collector.push(socket);
    }
    collector
}
