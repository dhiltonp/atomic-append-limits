use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::io::{stdout, Write};
use std::path::{Path, PathBuf};
use std::{env, fs, iter, process, thread, time};
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

fn random_chars(len: u32) -> String {
    let mut rng = thread_rng();
    let mut chars = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(len as usize - 1)
        .collect();
    chars += "\n";
    chars
}

fn spawn(id: u16, args: &Cli, process_path: &PathBuf) -> Result<process::Child, std::io::Error> {
    process::Command::new(&process_path)
        .arg("--worker-id")
        .arg(format!("{}", id))
        .arg("--file")
        .arg(&args.file)
        .arg("--processes")
        .arg("0") // in case there is a bug, don't spawn more children :)
        .arg("--times")
        .arg(format!("{}", args.times))
        .arg("--length")
        .arg(format!("{}", args.length))
        .spawn()
}

fn parent_process(args: &Cli) {
    let bytes = args.processes as u64 * args.times as u64 * args.length as u64 / 1024 / 1024;
    println!(
        "writing {}MB in {}B chunks across {} processes to {:?}",
        bytes, args.length, args.processes, args.file
    );
    println!("truncating {:?}", &args.file);
    fs::OpenOptions::new()
        .truncate(true)
        .write(true)
        .open(&args.file);

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
    println!("\npass complete");
}

fn child_process(args: &Cli) {
    print!("{} ", args.worker_id);
    stdout().flush();
    let chars = random_chars(args.length);
    let file = fs::OpenOptions::new()
        .read(false)
        .append(true)
        .create(true)
        .open(&args.file);
    if let Ok(mut file) = file {
        for i in 0..args.times {
            let written = file.write(chars.as_bytes()).unwrap();
            if written as u32 != args.length {
                panic!("partial write: {}B written", written);
            }
        }
    }
    print!("{} ", args.worker_id);
    stdout().flush();
}

fn main() {
    let args = Cli::from_args();
    if args.worker_id == 0 {
        parent_process(&args);
    } else {
        child_process(&args);
    }
}
