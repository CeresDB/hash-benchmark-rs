mod util;

use ahash::AHasher;
use seahash::SeaHasher;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::Hasher;
use std::time::{Duration, Instant};
use util::MurmurHasher;

use crate::util::gen_random_string;

const KEY_NUM: usize = 10_000_000;
const KEY_LEN: usize = 100;
const BUCKET_LEN: usize = 128;

fn test_speed<H: Hasher + Default>(keys: &[String]) -> Duration {
    let start_time = Instant::now();
    for key in keys {
        let mut hasher = H::default();
        hasher.write(key.as_bytes());
        hasher.finish();
    }

    start_time.elapsed()
}

fn test_collisions<H: Hasher + Default>(keys: &[String]) -> usize {
    let mut dedup = HashSet::with_capacity(keys.len());
    for key in keys {
        let mut hasher = H::default();
        hasher.write(key.as_bytes());
        dedup.insert(hasher.finish());
    }

    keys.len() - dedup.len()
}

fn test_distribution<H: Hasher + Default>(keys: &[String]) -> f64 {
    let mut buckets = vec![0; BUCKET_LEN];
    for key in keys {
        let mut hasher = H::default();
        hasher.write(key.as_bytes());
        let idx = hasher.finish() as usize % BUCKET_LEN;
        buckets[idx] += 1;
    }

    let mean = buckets.iter().sum::<usize>() as f64 / BUCKET_LEN as f64;
    let variance = buckets
        .iter()
        .map(|n| {
            let diff = *n as f64 - mean;
            diff * diff
        })
        .sum::<f64>()
        / BUCKET_LEN as f64;

    // std_dev
    variance.sqrt()
}

fn main() {
    let keys: Vec<_> = (0..KEY_NUM).map(|_| gen_random_string(KEY_LEN)).collect();
    println!(
        "## Random string, key_num({}), key_len({})",
        KEY_NUM, KEY_LEN
    );
    run(&keys);

    let keys: Vec<_> = (0..KEY_NUM).map(|i| i.to_string()).collect();
    println!("\n\n ## Increasing number");
    run(&keys)
}

fn run(keys: &[String]) {
    // Current print as markdown table, maybe we can add more format
    // https://github.com/phsym/prettytable-rs/

    let as_ms = |v| -> f64 { v as f64 / 1000_f64 };

    println!("| Op | Default | AHash | Murmur | SeaHasher |");
    println!("| --- | --- | --- | --- | --- |");
    println!(
        "| build time | {:.3} | {:.3} | {:.3} | {:.3} |",
        as_ms(test_speed::<DefaultHasher>(keys).as_micros()),
        as_ms(test_speed::<AHasher>(keys).as_micros()),
        as_ms(test_speed::<MurmurHasher>(keys).as_micros()),
        as_ms(test_speed::<SeaHasher>(keys).as_micros()),
    );
    println!(
        "| std dev | {:.3} | {:.3} | {:.3} | {:.3} |",
        test_distribution::<DefaultHasher>(keys),
        test_distribution::<AHasher>(keys),
        test_distribution::<MurmurHasher>(keys),
        test_distribution::<SeaHasher>(keys),
    );

    println!(
        "| collision | {} | {} | {} | {} |",
        test_collisions::<DefaultHasher>(keys),
        test_collisions::<DefaultHasher>(keys),
        test_collisions::<DefaultHasher>(keys),
        test_collisions::<DefaultHasher>(keys),
    );
}
