// print command stdout and stderr in real time

use std::process::Stdio;
use tokio::process::Command;
use tokio::io::BufReader;
// use std::io::BufReader;
// use tokio_util::codec::{FramedRead, LinesCodec};
use tokio::io::AsyncBufReadExt;

#[tokio::main]
async fn main() {
    // get program name to run from command line
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 || args[1] == "-h" || args[1] == "--help" {
        println!("Usage: {} <program>", args[0]);
        std::process::exit(1);
    }

    let program = &args[1];

    // rest are args
    let program_args = &args[2..];

    println!("run: {} {}", program, program_args.join(" "));

    let mut cmd = Command::new(program);
    cmd.args(program_args);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd.spawn().expect("failed to spawn command");

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let mut stdout_reader = BufReader::new(stdout);
    let mut stderr_reader = BufReader::new(stderr);
    let mut stdout_line = String::new();
    let mut stderr_line = String::new();

    loop {
        let stdout_read = stdout_reader.read_line(&mut stdout_line);
        let stderr_read = stderr_reader.read_line(&mut stderr_line);

        tokio::select! {
            Ok(_) = stdout_read => {
                if stdout_line.is_empty() {
                    break;
                }
                print!("stdout: {}", stdout_line);
                stdout_line.clear();
            }
            Ok(_) = stderr_read => {
                if stderr_line.is_empty() {
                    break;
                }
                print!("stderr: {}", stderr_line);
                stderr_line.clear();
            }
        }
    }

    // let mut stdout_reader = FramedRead::new(stdout, LinesCodec::new());
    // let mut stderr_reader = FramedRead::new(stderr, LinesCodec::new());

    // while let Some(line) = stdout_reader.next().await {
    //     println!("stdout: {}", line.unwrap());
    // }

    // for line in stderr_reader.next().await {
    //     println!("stderr: {}", line.unwrap());
    // }

    let status = child.wait().await.expect("failed to wait on child");
    println!("child status: {}", status);
}
