use std::path::{Path, PathBuf};
use std::{env, process, thread, time};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// Attempt to induce file mangling when writing in append mode
struct Cli {
    #[structopt(short, long, default_value = "/tmp/atomic-appends.txt")]
    /// file that will be used as a write target
    file: PathBuf,

    #[structopt(short, long, default_value = "20")]
    /// number of processes that will write to the file
    processes: u16,

    #[structopt(short, long, default_value = "5000")]
    /// number of times each process will write
    times: u32,

    #[structopt(short, long, default_value = "4096")]
    /// length of each write
    length: u32,

    #[structopt(long, default_value = "0")]
    /// run as a worker (used by the main process)
    worker_id: u16,
}

fn spawn(id: u16, args: &Cli, process_path: &PathBuf) -> Result<process::Child, std::io::Error> {
    process::Command::new(&process_path)
        .arg("--worker-id")
        .arg(format!("{}", id))
        .arg("--file")
        .arg(&args.file)
        .arg("--processes")
        .arg("0")  // in case there is a bug, don't spawn more children :)
        .arg("--times")
        .arg(format!("{}", args.times))
        .arg("--length")
        .arg(format!("{}", args.length))
        .spawn()
}

fn main() {
    let args = Cli::from_args();
    if args.worker_id == 0 {
        let bytes = args.processes as u32 * args.times * args.length / 1024 / 1024;
        println!(
            "writing {}MB in {}B chunks across {} processes to {:?}",
            bytes, args.length, args.processes, args.file
        );
        println!("starting subprocesses");
        let process_path = env::current_exe().unwrap();
        let mut children = Vec::new();
        for i in 1..args.processes + 1 {
            if let Ok(child) = spawn(i, &args, &process_path) {
                children.push(child);
            } else {
                println!("error starting child #{}", i);
            }
        }
        thread::sleep(time::Duration::from_millis(250));
        println!("\nwaiting for subprocesses");
        for mut child in children {
            child.wait();
        }
    } else {
        print!("{} ", args.worker_id);
    }
}
