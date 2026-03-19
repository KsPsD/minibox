//! MiniBox CLI — Command-line interface for MiniBox
//!
//! Usage:
//!   minibox run <image> <command>
//!   minibox ps
//!   minibox stop <container-id>
//!   minibox images
//!   minibox pull <image>

use minibox_runtime::version;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "version" => println!("minibox v{}", version()),
        "run" => println!("TODO: minibox run"),
        "ps" => println!("TODO: minibox ps"),
        "stop" => println!("TODO: minibox stop"),
        "images" => println!("TODO: minibox images"),
        "pull" => println!("TODO: minibox pull"),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
        }
    }
}

fn print_usage() {
    println!(
        r#"🦀 MiniBox v{} — Lightweight Container Runtime

Usage:
  minibox run <image> <command>    Run a container
  minibox ps                       List containers
  minibox stop <id>                Stop a container
  minibox images                   List images
  minibox pull <image>             Pull an image
  minibox version                  Show version"#,
        version()
    );
}
