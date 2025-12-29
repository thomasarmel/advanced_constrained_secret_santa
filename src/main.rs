use std::env;
use std::fs;
use std::process;
use std::str::FromStr;
use constrained_advanced_secret_santa::config::Config;
use constrained_advanced_secret_santa::santa_engine::SantaEngine;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <config_file.json5>", args[0]);
        process::exit(1);
    }
    let config_path = &args[1];
    let config_data = fs::read_to_string(config_path).unwrap_or_else(|err| {
        eprintln!("Cannot read JSON5 file '{}': {}", config_path, err);
        process::exit(1);
    });
    let config = Config::from_str(&config_data).unwrap_or_else(|err| {
        eprintln!("Cannot parse JSON5 file: {}", err);
        process::exit(1);
    });

    let engine = SantaEngine::new(&config);

    if let Some(cycles) = engine.generate() {
        for i in 0..engine.participants.len() {
            let d = &engine.participants[i];
            let mut receivers = Vec::new();

            for cycle in &cycles {
                if let Some(pos) = cycle.iter().position(|&id| id == i) {
                    let next_id = cycle[(pos + 1) % cycle.len()];
                    receivers.push(engine.participants[next_id].name.clone());
                }
            }

            println!("{} 🎁 -> {}", d.name, receivers.join(", "));
        }
    } else {
        eprintln!("No valid solution found. Please check your constraints.");
        process::exit(1);
    }
}