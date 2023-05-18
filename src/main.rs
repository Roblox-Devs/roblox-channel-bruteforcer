use tokio::sync::mpsc::{channel};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::fs::File;
use tokio::io::BufReader;
use tokio::task;
use reqwest::Client;
use ansi_term::Colour::{Red, Green, Purple};
use structopt::StructOpt;
use tokio::sync::Mutex;
use std::sync::Arc;
use std::convert::TryInto;

#[derive(StructOpt)]
#[structopt(name = "rbx channel bruteforcer", version = "1.1")]
struct Opt {
    #[structopt(long, help = "wordlist to use", display_order = 1)]
    wordlist: String,

    #[structopt(long, help = "prefix for channel name (ex: zproject)", display_order = 2)]
    prefix: String,

    #[structopt(long, help = "number of worker threads you want used", default_value = "250", display_order = 4)]
    threads: u32,

    #[structopt(long, help = "name of output file", display_order = 3)]
    output: String,
}
#[tokio::main]
async fn main() {
    if let Err(err) = enable_ansi_support::enable_ansi_support() {
        eprintln!("Failed to enable ANSI support: {:?}", err);
        return;
    }

    let opt = Opt::from_args();
    let file = File::open(&opt.wordlist).await.expect("Unable to open wordlist");
    let reader = BufReader::new(file);
    let client = Client::new();

    let (tx, rx) = channel(opt.threads.try_into().unwrap());
    let num_threads = opt.threads;

    let mut tasks = Vec::new();

    let rx = Arc::new(Mutex::new(rx));

    for _ in 0..num_threads {
        let rx = Arc::clone(&rx);

        let client = client.clone();
        let prefix = opt.prefix.clone();
        let output = opt.output.clone();

        let task = task::spawn(async move {
            loop {
                let line = {
                    let mut rx_guard = rx.lock().await;
                    rx_guard.recv().await
                };

                if let Some(line) = line {
                    let url = format!("https://setup.rbxcdn.com/channel/{}{}/DeployHistory.txt", prefix, line);

                    match get_body(&client, &url).await {
                        Ok(response) => {
                            if response.status().is_success() {
                                let mut file_ref = tokio::fs::OpenOptions::new()
                                    .append(true)
                                    .open(&output)
                                    .await
                                    .expect("Unable to open file");
                                file_ref.write_all(&url.as_bytes().to_owned().into_iter().chain("\n".as_bytes().to_owned().into_iter()).collect::<Vec<_>>()).await.expect("Unable to write to file");

                                eprintln!("{} {}", Green.paint("[SUCCESS]:"), url);
                            } else {
                                eprintln!("{} {}", Red.paint("[FAILURE]:"), url);
                            }
                        }
                        Err(_) => {
                            eprintln!("{} {}", Purple.paint("[UNREACHABLE]:"), url);
                        }
                    }
                } else {
                    break;
                }
            }
        });

        tasks.push(task);
    }

    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await.expect("Failed to read line") {
        tx.send(line).await.expect("Failed to send line to worker");
    }

    drop(tx);

    for task in tasks {
        task.await.expect("Thread panicked");
    }
}

async fn get_body(client: &Client, url: &str) -> Result<reqwest::Response, reqwest::Error> {
    let response = client
        .get(url)
        .send()
        .await?;

    Ok(response)
}
