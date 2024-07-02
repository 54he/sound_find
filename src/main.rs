use tokio::net::{TcpListener,TcpStream};
use std::fs::{File,OpenOptions};
use std::io::Write;
use chrono::Local;
use std::io::BufReader;
use rodio::Source;
use std::time::Duration;
use std::time::Instant;
use std::thread::sleep;
#[tokio::main]
async fn main(){
    let listener = TcpListener::bind("0.0.0.0:9696").await.unwrap();
    
    loop {
        let (client, _client_sock_addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
              log_write(client);play_sound();
        });
    }
}
fn log_write(client:TcpStream){
    let fmt = "%Y-%m-%d %H:%M:%S";
    let now = Local::now().format(fmt);
    println!("[{}]:{:?}",now,client);
    let file = "/var/log/sound_find/connect.log";
    let output = OpenOptions::new().append(true).open(file);
    write!(output.expect("不能追加写入，请检查/var/log/sound_find/connect.log的写入权限和该文件是否创建若没有请自行创建目录及其文件"), "\n[{}]:{:?}",now,client).unwrap();
    
}
fn play_sound(){
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    // 使用Cargo.toml文件所在的相对路径加载音频文件
    let file = File::open("/root/sound_find/output.ogg").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    stream_handle.play_raw(source.convert_samples()).expect("无法播放!");
    // 声音会通过一个独立的音频线程播放，
    // 所以我们需要在音频播放过程中保证主线程没有关闭。
    // 完成后，按 ctrl + C 强制关闭进程。
    sleep(Duration::new(3, 0))
}
