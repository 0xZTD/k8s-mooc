use std::{thread, time::Duration};

use chrono::{SecondsFormat, Utc};
use rand::distr::{Alphanumeric, SampleString};

fn main() {
    let generated_str = gen_rand_str(64);

    loop {
        println!("{}: {}", gen_timestamp(), generated_str);
        thread::sleep(Duration::from_secs(5));
    }
}

fn gen_rand_str(len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::rng(), len)
}

fn gen_timestamp() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}
