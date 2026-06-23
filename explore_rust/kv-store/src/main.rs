use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use anyhow::Result;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

type Store = Arc<RwLock<HashMap<String, String>>>;

#[tokio::main]
async fn main() -> Result<()> {
    // For now hardcode address; later we can use env/CLI args
    let addr = "127.0.0.1:6379";
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on {}", addr);

    let store: Store = Arc::new(RwLock::new(HashMap::new()));

    loop {
        let (socket, peer_addr) = listener.accept().await?;
        println!("New connection from {}", peer_addr);

        let store = Arc::clone(&store);

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, store).await {
                eprintln!("Error handling {}: {:?}", peer_addr, e);
            } else {
                println!("Connection {} closed", peer_addr);
            }
        });
    }
}

async fn handle_client(stream: TcpStream, store: Store) -> Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    writer
        .write_all(b"Welcome to rust-kv. Commands: SET/GET/DEL.\n")
        .await?;

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).await?;

        if bytes_read == 0 {
            // EOF / connection closed
            break;
        }

        // Trim newline
        let line_trimmed = line.trim();
        if line_trimmed.is_empty() {
            continue;
        }

        let mut parts = line_trimmed.splitn(3, ' '); // max 3 pieces: cmd key value

        let cmd = parts.next().unwrap().to_uppercase();
        match cmd.as_str() {
            "SET" => {
                let key = match parts.next() {
                    Some(k) => k.to_string(),
                    None => {
                        writer.write_all(b"-ERR missing key\r\n").await?;
                        continue;
                    }
                };
                let value = match parts.next() {
                    Some(v) => v.to_string(),
                    None => {
                        writer.write_all(b"-ERR missing value\r\n").await?;
                        continue;
                    }
                };

                {
                    let mut map = store.write().unwrap();
                    map.insert(key, value);
                }

                writer.write_all(b"+OK\r\n").await?;
            }

            "GET" => {
                let key = match parts.next() {
                    Some(k) => k,
                    None => {
                        writer.write_all(b"-ERR missing key\r\n").await?;
                        continue;
                    }
                };

                let value_opt = {
                    let map = store.read().unwrap();
                    map.get(key).cloned()
                };

                match value_opt {
                    Some(v) => {
                        // Simple protocol: return value + newline
                        writer
                            .write_all(format!("${}\r\n", v).as_bytes())
                            .await?;
                    }
                    None => {
                        writer.write_all(b"$nil\r\n").await?;
                    }
                }
            }

            "DEL" => {
                let key = match parts.next() {
                    Some(k) => k,
                    None => {
                        writer.write_all(b"-ERR missing key\r\n").await?;
                        continue;
                    }
                };

                let removed = {
                    let mut map = store.write().unwrap();
                    map.remove(key).is_some()
                };

                if removed {
                    writer.write_all(b":1\r\n").await?;
                } else {
                    writer.write_all(b":0\r\n").await?;
                }
            }

            "QUIT" => {
                writer.write_all(b"+BYE\r\n").await?;
                break;
            }

            _ => {
                writer
                    .write_all(b"-ERR unknown command. Use SET/GET/DEL/QUIT\r\n")
                    .await?;
            }
        }
    }

    Ok(())
}
