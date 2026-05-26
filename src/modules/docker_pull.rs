use async_trait::async_trait;
use rand::seq::IndexedRandom;
use rand::{RngExt, rng};

use crate::args::AppConfig;
use crate::data::DOCKER_IMAGES_LIST;
use crate::generators::gen_hex_string;
use crate::io::{csleep, erase_line, get_terminal_width, newline, print};
use crate::modules::Module;

pub struct DockerPull;

#[async_trait(?Send)]
impl Module for DockerPull {
    fn name(&self) -> &'static str {
        "docker_pull"
    }

    fn signature(&self) -> String {
        "docker pull pytorch/pytorch:latest".to_string()
    }

    async fn run(&self, appconfig: &AppConfig) {
        let mut rng = rng();

        let image = DOCKER_IMAGES_LIST.choose(&mut rng).unwrap_or(&"ubuntu");
        let tag = [
            "latest", "nightly", "2.4.0", "1.15.0", "24.04", "22.04", "lts", "slim", "alpine",
            "bookworm",
        ]
        .choose(&mut rng)
        .unwrap_or(&"latest");

        print(format!("Using default tag: {tag}")).await;
        newline().await;
        print(format!("{image}:{tag}")).await;
        newline().await;

        csleep(rng.random_range(300..1500)).await;

        let num_layers = rng.random_range(6..18);
        let layer_hashes: Vec<String> = (0..num_layers)
            .map(|_| format!("sha256:{}", gen_hex_string(&mut rng, 12)))
            .collect();

        for (i, layer_hash) in layer_hashes.iter().enumerate() {
            if appconfig.should_exit() {
                return;
            }

            // Early layers are base image layers (smaller), later ones are bigger
            let layer_size = if i < 3 {
                rng.random_range(500_000..5_000_000)
            } else if i < num_layers - 2 {
                rng.random_range(10_000_000..300_000_000)
            } else {
                rng.random_range(300_000..3_000_000)
            };

            // Simulate varying connection speeds per layer
            let connection_speed = rng.random_range(2..12);

            let terminal_width = get_terminal_width();
            let progress_bar_width = terminal_width.saturating_sub(55).max(10);
            let mut progress_bar = progress_string::BarBuilder::new()
                .total(layer_size as usize)
                .full_char('=')
                .width(progress_bar_width)
                .build();

            let mut downloaded: u64 = 0;
            loop {
                let chunk = ((layer_size as f64 / 100.0) * connection_speed as f64 / 20.0) as u64;
                let chunk = rng.random_range((chunk / 2).max(1)..(chunk * 2).max(2));
                let chunk = chunk.min((layer_size as u64 - downloaded).max(1));
                let chunk = chunk.max(1).min(10_000_000);
                downloaded = (downloaded + chunk).min(layer_size as u64);
                let percent = (downloaded as f64 / layer_size as f64 * 100.0).min(100.0);

                erase_line().await;
                progress_bar.replace(downloaded as usize);
                let display_hash = &layer_hash[7..19];

                print(format!(
                    "{digest}: Downloading  {progress_bar} {downloaded:>9} / {total:>9}  {percent:>3.0}%",
                    digest = display_hash,
                    progress_bar = progress_bar,
                    downloaded = humansize::format_size(downloaded, humansize::BINARY),
                    total = humansize::format_size(layer_size as u64, humansize::BINARY),
                    percent = percent,
                ))
                .await;

                if downloaded >= layer_size as u64 {
                    break;
                }

                csleep(rng.random_range(80..300)).await;
            }

            newline().await;
            let digest = &layer_hash[7..19];
            print(format!("{digest}: Extracting")).await;
            csleep(rng.random_range(500..3000)).await;
            erase_line().await;
            print(format!("{digest}: Pull complete")).await;
            newline().await;
        }

        let full_hash = gen_hex_string(&mut rng, 64);
        print(format!("Digest: sha256:{full_hash}")).await;
        newline().await;
        print(format!("Status: Downloaded newer image for {image}:{tag}",)).await;
        newline().await;
        newline().await;
        print(format!(
            "What's next:\n    docker run --gpus all -it --rm {image}:{tag} bash",
        ))
        .await;
        newline().await;
    }
}
