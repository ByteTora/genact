use async_trait::async_trait;
use rand::seq::IndexedRandom;
use rand::{RngExt, rng};
use yansi::Paint;

use crate::args::AppConfig;
use crate::data::LLM_TERMS_LIST;
use crate::io::{csleep, dprint, erase_line, newline, print};
use crate::modules::Module;

pub struct LlmTrain;

fn format_mem(bytes: f64) -> String {
    let gb = bytes / 1_073_741_824.0;
    format!("{gb:.1}GiB")
}

#[async_trait(?Send)]
impl Module for LlmTrain {
    fn name(&self) -> &'static str {
        "llm_train"
    }

    fn signature(&self) -> String {
        "torchrun --nproc_per_node=8 train.py --model Llama-7B".to_string()
    }

    async fn run(&self, appconfig: &AppConfig) {
        let mut rng = rng();

        let model_names = [
            "Llama-7B",
            "Llama-13B",
            "Llama-70B",
            "Mistral-7B",
            "Mistral-8x7B",
            "Qwen2-72B",
            "DeepSeek-67B",
            "GPT-NeoX-20B",
            "Falcon-40B",
            "Phi-3-mini",
            "Gemma-7B",
            "Yi-34B",
        ];
        let model = model_names.choose(&mut rng).unwrap_or(&"Llama-7B");

        dprint(
            format!(
                "{} Initializing distributed training on 8 GPUs...",
                Paint::cyan("INFO").bold()
            ),
            10,
        )
        .await;
        newline().await;
        csleep(rng.random_range(200..800)).await;
        dprint(
            format!(
                "{} Loading pretrained weights for {model}",
                Paint::cyan("INFO").bold()
            ),
            5,
        )
        .await;
        newline().await;
        csleep(rng.random_range(300..1000)).await;

        let num_frozen = rng.random_range(20..60);
        dprint(
            format!(
                "{} Freezing {} out of {} transformer layers (LoRA tuning)",
                Paint::yellow("WARN").bold(),
                num_frozen,
                num_frozen + rng.random_range(4..16),
            ),
            5,
        )
        .await;
        newline().await;

        let num_terms = rng.random_range(15..30);
        let terms: Vec<_> = LLM_TERMS_LIST.sample(&mut rng, num_terms).collect();
        dprint(
            format!(
                "{} Initializing {} parameter groups",
                Paint::cyan("INFO").bold(),
                terms.len()
            ),
            5,
        )
        .await;
        newline().await;

        for term in &terms {
            let param_count = rng.random_range(100_000..50_000_000);
            dprint(
                format!(
                    "{}   {:<55} {} params",
                    Paint::cyan("INFO").bold(),
                    format!("{term}:"),
                    humansize::format_size(param_count as u64, humansize::BINARY),
                ),
                2,
            )
            .await;
            newline().await;
        }

        csleep(500).await;
        dprint(
            format!(
                "{} Total trainable params: {}",
                Paint::cyan("INFO").bold(),
                humansize::format_size(
                    rng.random_range(500_000_000..8_000_000_000u64),
                    humansize::BINARY
                ),
            ),
            5,
        )
        .await;
        newline().await;
        newline().await;

        let total_epochs = rng.random_range(3..6);
        let steps_per_epoch = rng.random_range(120..400);
        let base_loss = rng.random_range(4.0..9.0);
        let gpu_mem_total = 80.0;
        let mut loss: f64 = base_loss;
        let mut best_loss: f64 = loss;

        for epoch in 1..=total_epochs {
            if appconfig.should_exit() {
                return;
            }

            dprint(
                format!(
                    "{} ------------ Epoch {epoch}/{total_epochs} ------------",
                    Paint::blue("TRAIN").bold(),
                ),
                3,
            )
            .await;
            newline().await;

            for step in 1..=steps_per_epoch {
                if appconfig.should_exit() {
                    return;
                }

                let progress = step as f64 / steps_per_epoch as f64;
                let bar_filled = ((progress * 50.0) as usize).min(50);
                let bar_empty = 50 - bar_filled;

                // Simulate stochastic loss descent: occasional spikes
                if rng.random_bool(0.05) {
                    loss += rng.random_range(0.1..0.5);
                } else {
                    loss -= rng.random_range(0.005..0.08) * (loss * 0.04).max(0.001);
                }
                loss = loss.max(0.01);
                best_loss = best_loss.min(loss);

                let lr = 3e-5
                    * (1.0
                        - ((epoch - 1) as f64 * steps_per_epoch as f64 + step as f64)
                            / (total_epochs as f64 * steps_per_epoch as f64)
                            * 0.9)
                        .max(0.1);

                let grad_norm = rng.random_range(0.05..3.5);
                let gpu_used: f64 = 40.0 + rng.random_range(0.0..12.0) + (loss * 2.5).min(25.0);
                let gpu_used = gpu_used.min(gpu_mem_total);
                let mem_used = format_mem(gpu_used * 1_073_741_824.0);
                let mem_total = format_mem(gpu_mem_total * 1_073_741_824.0);

                let throughput = rng.random_range(1200..4800);

                erase_line().await;
                print(format!(
                    "  {progress_bar:50} step {step:>4}/{steps_per_epoch}  |  loss: {loss:<8.4}  lr: {lr:<.2e}  |  grad_norm: {grad_norm:<5.2}  mem: {mem_used}/{mem_total}  |  {throughput} tok/s",
                    progress_bar = format!("[{}{}]", "=".repeat(bar_filled), " ".repeat(bar_empty)),
                ))
                .await;

                let sleep = if rng.random_bool(0.03) {
                    // Simulate data loading stalls
                    rng.random_range(800..2000)
                } else if step < 5 {
                    // Early steps often slower (warmup)
                    rng.random_range(300..800)
                } else {
                    rng.random_range(100..600)
                };
                csleep(sleep).await;
            }

            newline().await;
            newline().await;

            dprint(
                format!(
                    "{} Epoch {epoch} completed. loss: {loss:.4}, best_loss: {best_loss:.4}",
                    Paint::green("SAVE").bold(),
                ),
                5,
            )
            .await;
            newline().await;

            dprint(
                format!(
                    "{} Saving checkpoint to ./checkpoints/epoch_{epoch}.pt",
                    Paint::green("SAVE").bold()
                ),
                8,
            )
            .await;
            newline().await;
            csleep(rng.random_range(300..1200)).await;

            // Validation phase
            dprint(
                format!("{} Running validation...", Paint::cyan("INFO").bold()),
                3,
            )
            .await;
            newline().await;
            let val_steps = rng.random_range(10..30);
            for v in 1..=val_steps {
                if appconfig.should_exit() {
                    return;
                }
                let val_loss = loss + rng.random_range(0.05..0.3);
                erase_line().await;
                print(format!(
                    "  Validating... batch {v}/{val_steps}  |  val_loss: {val_loss:.4}",
                ))
                .await;
                csleep(rng.random_range(50..200)).await;
            }
            newline().await;
            csleep(rng.random_range(200..800)).await;
            newline().await;
        }

        dprint(
            format!("{} Training completed!", Paint::green("DONE").bold()),
            10,
        )
        .await;
        newline().await;
        dprint(
            format!("{} Best loss: {best_loss:.4}", Paint::cyan("INFO").bold()),
            5,
        )
        .await;
        newline().await;
        dprint(
            format!(
                "{} Saving final model to ./output/{model}_final.pt",
                Paint::cyan("INFO").bold()
            ),
            8,
        )
        .await;
        newline().await;
        dprint(
            format!(
                "{} Uploading to Hugging Face Hub: svenstaro/{model}",
                Paint::cyan("INFO").bold()
            ),
            10,
        )
        .await;
        newline().await;
    }
}
