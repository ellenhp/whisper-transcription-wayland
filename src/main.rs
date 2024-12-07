use std::time::Duration;

use audio::record_wav;
use enigo::{Direction, Enigo, Key, Keyboard};
use reqwest::StatusCode;
use tempfile::TempDir;
use tokio::time::sleep;

mod audio;

#[tokio::main]
async fn main() {
    env_logger::init();

    let dir = TempDir::new().expect("Failed to create temp dir");
    let wav_path = dir.path().join("recording.wav");
    record_wav(&wav_path).expect("Failed to record wav");
    let audio_data = std::fs::read(wav_path).expect("Failed to read wav file");

    dbg!(audio_data.len());

    let client = reqwest::Client::new();
    let request = client.post("http://127.0.0.1:8009/transcribe")
        .body(audio_data)
        .build()
        .unwrap();

    let text = match client.execute(request).await {
        Ok(response) => {
            if response.status() == StatusCode::OK {
                response.text().await.expect("Failed to get text")
            } else {
                eprintln!("Request returned non-200 {:?}", response.status());
                return;
            }
        },
        Err(err) => {
            eprintln!("Request failed {}", err);
            return;
        },
    };

    // Simulate typing
    let mut enigo = Enigo::new(&Default::default()).unwrap();
    for c in text.trim().chars() {
        enigo.key(Key::Unicode(c), Direction::Click).expect("Failed to type key");
        sleep(Duration::from_millis(5)).await;
    }
    enigo.key(Key::Space, Direction::Click).expect("Failed to type key");
}
