//! CLI tool for ternary-rhythm: generate, analyze, and visualize rhythm patterns.
//!
//! Usage:
//!   ternary-rhythm euclidean <k> <n>
//!   ternary-rhythm meter <beats> <note_value>
//!   ternary-rhythm analyze <pattern_string>
//!   ternary-rhythm swing <pattern_string> <amount>
//!   ternary-rhythm rotate <pattern_string> <shift>
//!   ternary-rhythm preset <name>

use std::env;
use std::process::exit;

use ternary_rhythm::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        exit(1);
    }

    match args[1].as_str() {
        "euclidean" => cmd_euclidean(&args),
        "meter" | "metre" => cmd_meter(&args),
        "analyze" | "analyse" | "classify" => cmd_analyze(&args),
        "swing" => cmd_swing(&args),
        "rotate" => cmd_rotate(&args),
        "preset" => cmd_preset(&args),
        "presets" | "list" => cmd_list_presets(),
        "help" | "--help" | "-h" => { print_usage(); }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
            exit(1);
        }
    }
}

fn print_usage() {
    eprintln!(r#"ternary-rhythm — Temporal pattern recognition using ternary {{-1,0,+1}} time patterns

USAGE:
    ternary-rhythm <command> [options]

COMMANDS:
    euclidean <k> <n>      Generate Euclidean rhythm E(k,n)
    meter <beats> <sub>    Generate meter pattern
    analyze <string>       Analyze a pattern (X=accent, o=ghost, .=silence)
    swing <string> <amt>   Apply swing (amt: 0.0-1.0)
    rotate <string> <n>    Rotate pattern by n positions
    preset <name>          Show a preset pattern
    presets                List available presets
    help                   This help

EXAMPLES:
    ternary-rhythm euclidean 3 8
    ternary-rhythm meter 4 4
    ternary-rhythm analyze "X..X..X."
    ternary-rhythm swing "X.X.X.X." 0.6
    ternary-rhythm rotate "X..X..X." 2
    ternary-rhythm preset bossa_nova
"#);
}

fn cmd_euclidean(args: &[String]) {
    if args.len() < 4 {
        eprintln!("Usage: ternary-rhythm euclidean <k> <n>");
        exit(1);
    }
    let k: usize = args[2].parse().unwrap_or_else(|_| { eprintln!("Invalid k"); exit(1); });
    let n: usize = args[3].parse().unwrap_or_else(|_| { eprintln!("Invalid n"); exit(1); });
    let pattern = euclidean(k, n);
    let label = format!("E({},{}) = {} hits in {} steps", k, n, k, n);
    print_pattern(&pattern, Some(&label));
}

fn cmd_meter(args: &[String]) {
    if args.len() < 4 {
        eprintln!("Usage: ternary-rhythm meter <beats> <subdivisions>");
        exit(1);
    }
    let beats: usize = args[2].parse().unwrap_or_else(|_| { eprintln!("Invalid beats"); exit(1); });
    let sub: usize = args[3].parse().unwrap_or_else(|_| { eprintln!("Invalid subdivisions"); exit(1); });
    let pattern = generate_meter(beats, sub);
    let label = format!("{}/{} meter", beats, sub);
    print_pattern(&pattern, Some(&label));
}

fn cmd_analyze(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: ternary-rhythm analyze <pattern_string>");
        exit(1);
    }
    let pattern = from_string(&args[2]);
    if pattern.is_empty() {
        eprintln!("Error: empty pattern (or all silence)");
        exit(1);
    }

    let class = classify(&pattern);
    println!("┌─ Rhythm Analysis ─────────────────────────────┐");
    println!("│ Pattern  : {}  │", pad(&to_string(&pattern), 32));
    println!("│ Length   : {:>2} steps                      │", pattern.len());
    println!("│ Meter    : {:<5}                               │", class.meter);
    println!("│ Feel     : {:<20}                    │", class.feel);
    println!("│ Genre    : {:<25}                 │", class.genre);
    println!("│ Density  : {:.3}                              │", density(&pattern));
    println!("│ Syncop.  : {:.3}                              │", class.syncopation);
    println!("│ Ghosts   : {:<5}                               │", if class.has_ghosts { "yes" } else { "no" });
    println!("└────────────────────────────────────────────────┘");
    println!();
    print_pattern(&pattern, Some("Pattern"));
}

fn cmd_swing(args: &[String]) {
    if args.len() < 4 {
        eprintln!("Usage: ternary-rhythm swing <pattern_string> <amount>");
        exit(1);
    }
    let pattern = from_string(&args[2]);
    let amount: f64 = args[3].parse().unwrap_or_else(|_| { eprintln!("Invalid amount"); exit(1); });
    let swung = swing(&pattern, amount);
    let label = format!("Swung (amount={:.2})", amount);
    println!("Original:");
    print_pattern(&pattern, None);
    println!();
    print_pattern(&swung, Some(&label));
}

fn cmd_rotate(args: &[String]) {
    if args.len() < 4 {
        eprintln!("Usage: ternary-rhythm rotate <pattern_string> <shift>");
        exit(1);
    }
    let pattern = from_string(&args[2]);
    let shift: isize = args[3].parse().unwrap_or_else(|_| { eprintln!("Invalid shift"); exit(1); });
    let rotated = rotate(&pattern, shift);
    print_pattern(&rotated, Some(&format!("Rotated by {}", shift)));
}

fn cmd_preset(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: ternary-rhythm preset <name>");
        eprintln!("Run 'ternary-rhythm presets' to list available presets.");
        exit(1);
    }
    let name = &args[2];
    let pattern = match name.as_str() {
        "rock" => presets::rock(),
        "waltz" => presets::waltz(),
        "shuffle" => presets::shuffle(),
        "bossa_nova" => presets::bossa_nova(),
        "tresillo" => presets::tresillo(),
        "funk" => presets::funk(),
        "techno" => presets::techno(),
        "five_four" | "5_4" => presets::five_four(),
        "balkan_seven" | "7_8" => presets::balkan_seven(),
        "afro_cuban_6_8" | "afro_cuban" => presets::afro_cuban_six_eight(),
        _ => {
            eprintln!("Unknown preset: {}", name);
            eprintln!("Run 'ternary-rhythm presets' to list available presets.");
            exit(1);
        }
    };
    print_pattern(&pattern, Some(name));
}

fn cmd_list_presets() {
    println!("Available presets:");
    println!("  rock              — Classic rock 4/4");
    println!("  waltz             — Waltz 3/4");
    println!("  shuffle           — Shuffle 12/8");
    println!("  bossa_nova        — Bossa nova clave");
    println!("  tresillo          — Latin tresillo");
    println!("  funk              — Funky drum break");
    println!("  techno            — Minimal techno");
    println!("  five_four (5_4)   — 5/4 time");
    println!("  balkan_seven (7_8) — 7/8 Balkan rhythm");
    println!("  afro_cuban_6_8    — Afro-Cuban 6/8 bell");
}

fn print_pattern(pattern: &Vec<Ternary>, label: Option<&str>) {
    println!("{}", visualize(pattern, label));
    println!("  String: {}", to_string(pattern));
}

fn pad(s: &str, width: usize) -> String {
    if s.len() >= width { s[..width].to_string() } else { format!("{}{}", s, " ".repeat(width - s.len())) }
}
