use std::env;
use std::fs;
use std::process;
use std::str::FromStr;
use std::collections::HashMap;
use base64::{Engine as _, engine::general_purpose};
use advanced_constrained_secret_santa::config::Config;
use advanced_constrained_secret_santa::santa_engine::SantaEngine;


const ADMIN_PASSWORD_SEE_ALL_GIFTS: &'static str = "perenoel";
const OUTPUT_HTML_FILE: &'static str = "santa_results.html";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <config_file.json5> [--to-html]", args[0]);
        process::exit(1);
    }

    let config_path = &args[1];
    let to_html = args.contains(&"--to-html".to_string());

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
        if to_html {
            println!();
            let output_html = generate_html(&engine, &cycles);
            fs::write(OUTPUT_HTML_FILE, output_html).expect("Unable to write HTML file");
            println!("HTML file generated: santa_results.html");
        }
    } else {
        eprintln!("No valid solution found. Please check your constraints.");
        process::exit(1);
    }
}

fn generate_html(engine: &SantaEngine, cycles: &Vec<Vec<usize>>) -> String {
    const TEMPLATE_HTML: &'static str = include_str!("template.html");

    let mut map = HashMap::new();

    for i in 0..engine.participants.len() {
        let d = &engine.participants[i];
        let mut receivers = Vec::new();
        for cycle in cycles {
            if let Some(pos) = cycle.iter().position(|&id| id == i) {
                let next_id = cycle[(pos + 1) % cycle.len()];
                receivers.push(engine.participants[next_id].name.clone());
            }
        }
        let encoded = general_purpose::STANDARD.encode(receivers.join(", "));
        map.insert(d.name.clone(), encoded);
    }

    let json_data = serde_json::to_string(&map).unwrap();
    let output = TEMPLATE_HTML
        .replace("{{DATA_PLACEHOLDER}}", &json_data)
        .replace("{{ADMIN_PASSWORD_SEE_ALL_GIFTS}}", ADMIN_PASSWORD_SEE_ALL_GIFTS);

    output
}