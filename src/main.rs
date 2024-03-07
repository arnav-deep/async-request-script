use std::{collections::HashMap, time::Instant};

use futures::{stream, StreamExt};
// can use the rand library as well for producing random data if required.
// use rand::Rng;
use reqwest::Client;
use tokio;

// how many requests to send concurrently
const CONCURRENT_REQUESTS: usize = 10; // Adjust as needed

#[tokio::main]
async fn main() {
    let client = Client::new();
    let urls = vec!["http://localhost:6001/"; 15]; // Replace with your actual endpoint URLs

    let bodies = stream::iter(urls)
        .map(|url| {
            let client = &client;
            async move {
                let mut json_body: HashMap<_, _> = HashMap::new();
                json_body.insert("num", 17);
                let resp = client
                    .post(url)
                    .json(&json_body.clone())
                    // Add your headers
                    // .header(
                    //     "Host",
                    //     "C3Y46V7TCCGSDHUMHUZS2JRCL5WDX7UZS3JBJRDGBCGYVK2N7NTQ.oyster.run",
                    // )
                    .send()
                    .await?;
                resp.text().await
            }
        })
        .buffer_unordered(CONCURRENT_REQUESTS);

    bodies
        .for_each(|b| async {
            let start_time = Instant::now();
            match b {
                Ok(response_text) => {
                    let elapsed_ms = start_time.elapsed().as_millis();
                    println!(
                        "Response: {}\nElapsed time: {}\n\n",
                        response_text, elapsed_ms
                    );
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        })
        .await;
}
