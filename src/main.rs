use std::sync::LazyLock;

use anyhow::Result;
use manager::ManagerAI;
use rust_dotenv::dotenv::DotEnv;
use tokio::io::AsyncReadExt;

pub static ENV: LazyLock<DotEnv> = LazyLock::new(|| DotEnv::new(""));

mod base;
mod manager;

#[tokio::main]
async fn main() -> Result<()> {
    let mut manager = ManagerAI::new();
    loop {
        let mut buf = Vec::new();
        loop {
            let mut sbuf = [0];
            let _ = tokio::io::stdin().read(&mut sbuf).await.unwrap();
            if sbuf[0] == b'\n' {
                break;
            } else {
                buf.push(sbuf[0]);
            }
        }
        let str = String::from_utf8(buf).unwrap();
        manager.orchestrate(str).await;
    }
}
