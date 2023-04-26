use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
use ureq::{Agent, AgentBuilder};
use colored::Colorize;
use std::io::{self, Write};
use ansi_term::Colour::Red;
use std::fs::OpenOptions;
use ansi_term::Colour::Green;
fn main() {
    enable_ansi_support::enable_ansi_support();
    let file = std::fs::File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let agent: Agent = AgentBuilder::new()
        .timeout_read(std::time::Duration::from_secs(5))
        .build();

    let lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();
    let lines_arc = Arc::new(Mutex::new(lines));

    let mut handles = vec![];

    for _ in 0..num_cpus::get() {
        let lines_arc = lines_arc.clone();
        let agent = agent.clone();

        let handle = thread::spawn(move || {
            loop {
                let line = {
                    let mut lines = lines_arc.lock().unwrap();
                    if lines.is_empty() {
                        break;
                    }
                    lines.pop().unwrap()
                };
                let url = format!(
                    "https://setup.rbxcdn.com/channel/z{}/DeployHistory.txt",
                    line
                );
                match get_body(&agent, &url) {
                    Ok(body) => {
                        //append to file named "success.txt"
                        let mut fileRef = OpenOptions::new().append(true).open("success.txt").expect("Unable to open file");   
                        fileRef.write_all(url.as_bytes()).expect("Unable to write to file");
                        fileRef.write_all("\n".as_bytes()).expect("Unable to write to file");

                        println!("SUCCESS: {} ", Green.paint(&url))
                    },
                    Err(err) => eprintln!("{} {}", Red.paint("[FAILED]:"), url),
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn get_body(agent: &Agent, url: &str) -> Result<String, ureq::Error> {
    let body: String = agent
        .get(url)
        .set("Example-Header", "header value")
        .call()?
        .into_string()?;
    Ok(body)
}
