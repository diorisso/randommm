use std::fs::{ReadDir, DirEntry};
use sysinfo::{System, SystemExt, Process};
use rand::Rng;
use std::process::Command;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'd', long = "dir")]
    directory: std::path::PathBuf
}

fn start_swww() {
    Command::new("swww")
        .arg("init")
        .spawn()
        .expect("error initializing swww :(");
}

fn swww_is_running() -> bool {
    let sys = System::new_all();

    let process = sys.processes_by_exact_name("swww");
    
    process.collect::<Vec<&Process>>().len() > 0
}

fn randomize(dir: ReadDir) {
    let vec = dir.map(|file| file.unwrap())
        .collect::<Vec<DirEntry>>();
    
    let random_num = rand::thread_rng()
        .gen_range(0..vec.len());
   
    let path = vec.get(random_num - 1)
        .unwrap()
        .path()
        .into_os_string()
        .into_string()
        .unwrap();

    Command::new("swww")
        .args(["img", &path])
        .spawn()
        .expect("there was an error setting the wallpaper, contact the devs :(");
} 

fn main() {
    let args = Cli::parse();
    let dir = std::fs::read_dir(&args.directory);

    match dir {
        Err(e) => println!("Error: {}", e),
        Ok(o) => {
            if !swww_is_running() { start_swww() }
            randomize(o);
        }
    }
}
