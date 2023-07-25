use clap::Parser;
use reqwest::Client;
use std::time::{Duration, Instant};
use std::{fs::File, io::Write};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    node_count: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut urls = vec![];
    for node_id in 0..args.node_count {
        let url = format!(
            "http://127.0.1:{}/hello",
            8080 + node_id
        );
        urls.push(url);
    }
    let num_requests = 1000;
    let delay = Duration::from_millis(10);
    let mut tasks = Vec::new();
    for i in 0..num_requests {
        let url =
            urls[i % args.node_count as usize].clone();
        tasks.push(tokio::spawn(async move {
            let url = url.clone();
            let client = Client::new();
            let begin = Instant::now();
            let response = client.get(url).send().await?;
            let end = Instant::now();
            if !response.status().is_success() {
                anyhow::bail!(
                    "HTTP request failed: {}",
                    response.status()
                );
            }
            let latency = end.duration_since(begin);
            Ok((i, latency.as_secs_f64()))
        }));
        tokio::time::sleep(delay).await;
    }
    let mut file = File::create("latency_samples.txt")?;
    writeln!(file, "Sample,Latency_secs")?;
    for task in tasks {
        let (sample, latency) = task.await??;
        writeln!(file, "{},{}", sample, latency)?;
    }
    Ok(())
}
