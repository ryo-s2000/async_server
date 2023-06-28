use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listner = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        print!("start loop\n");
        // stop loop until accept
        let (mut socket, addr) = listner.accept().await?;
        println!("accept {}\n", addr);

        // after tcp connected work async then use pool
        tokio::spawn(async move {
            let (r, w) = socket.split();
            let mut reader = io::BufReader::new(r);
            let mut writer = io::BufWriter::new(w);

            let mut line = String::new();
            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) => {
                        println!("closed: {}", addr);
                        return;
                    }
                    Ok(_) => {
                        print!("{} {}", addr, line);
                        writer.write_all(line.as_bytes()).await.unwrap();
                        writer.flush().await.unwrap();
                    }
                    Err(e) => {
                        println!("{} {}", addr, e);
                        return;
                    }
                }
            }
        });
    }
}
