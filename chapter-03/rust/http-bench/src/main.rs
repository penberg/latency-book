use reqwest::Client;
use std::time::{Duration, Instant};
use std::{fs::File, io::Write};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "http://127.0.1:8080/hello";
    let num_requests = 1000;
    let delay = Duration::from_millis(10);
    let mut tasks = Vec::new();
    for i in 0..num_requests {
        tasks.push(tokio::spawn(async move {
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
