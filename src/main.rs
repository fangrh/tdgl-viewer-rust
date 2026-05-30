use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod dataset;
mod iv_curve;
mod manifest;
mod scanner;
mod timeline;

#[derive(Parser)]
#[command(name = "tdgl-viewer", about = "TDGL simulation viewer")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Watch { run_dir: PathBuf },
    Iv { run_dir: PathBuf },
    Info { run_dir: PathBuf },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Watch { run_dir } => {
            let sc = scanner::Scanner::new(&run_dir).expect("Failed to create scanner");
            println!("Watching {} ({} Je files found)", run_dir.display(), sc.count());
            println!("Waiting for new files...");
            sc.watch(|je_file| {
                println!("New Je file: {} (Je={:.2})", je_file.path.display(), je_file.je_value);
            }).expect("Watch failed");
        }
        Commands::Iv { run_dir } => {
            let sc = scanner::Scanner::new(&run_dir).expect("Failed to create scanner");
            let iv = iv_curve::IVCurve::compute(&sc).expect("Failed to compute I-V curve");
            println!("I-V Curve ({} points):", iv.points.len());
            println!("{:>12} {:>12} {:>12}", "Je", "Voltage", "Resistance");
            for p in &iv.points {
                println!("{:12.4e} {:12.6e} {:12.6e}", p.je_applied, p.voltage, p.resistance);
            }
        }
        Commands::Info { run_dir } => {
            let m = manifest::Manifest::load(&run_dir).expect("Failed to load manifest");
            println!("Run: {}", m.run_id);
            println!("Status: {}", m.scan.status);
            println!("Completed: {} / {}", m.scan.completed.len(), m.scan.Je_values.len());
            println!("Current Je: {:.2e}", m.scan.current_Je);
            println!("Pending: {:?}", m.pending());
        }
    }
}
