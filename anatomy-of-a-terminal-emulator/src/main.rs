/// Coding along with the blog post Anatomy of a Terminal Emulator by Aram Drevekenin
/// Code examples and comments are borrowed from https://poor.dev/terminal-anatomy/
/// Defining some terms as used in the blog post:
/// 
/// terminal emulator & shell are executable applications that run on our machine
/// 
/// terminal emulator :: graphical application, interprets data coming from the shell
///     and displays it on screen
/// 
/// shell :: provides an interface to the operating system, allows interactin with its file-system
/// 
/// pty (pseudoterminal) :: connects terminal emulator & shell
///     & provides a bi-directional asynchronous communication channel (STDIN & STDOUT)
/// 
/// STDIN :: represents the communication directed from the terminal emulator to the shell
/// STDOUT :: refers to the communication directed from the shell to the terminal emulator
/// 
/// “primary” and “secondary” side of the pty ::
/// -> on the primary side is the terminal emulator
/// -> on the secondary side is the shell
/// 
/// now to the default program ::
/// -> a program that spawns a pty & starts the machine’s default shell on its secondary side

// forkpty is a libc function that forks the current process
// it starts a pty and places the child part of the fork on the secondary side of the pty
use nix::pty::forkpty;
use nix::unistd::{
    ForkResult, 
    read
};
use std::os::fd::RawFd;
use std::process::Command;

// reading from the STDOUT file descriptor
// accepting the file descriptor that we got from the spawn_pty_with_shell
fn read_from_fd(fd: RawFd) -> Option<Vec<u8>> {
    // https://linux.die.net/man/7/pipe
    let mut read_buffer = [0; 65536];
    // sending the file descriptor to the read system call along with a mutable buffer
    let read_result = read(fd, &mut read_buffer);
    match read_result {
        // read() will read up to that amount of bytes
        // then place them in the buffer we gave it
        // and return us the number of bytes read
        // Ok(...) allocates a vector from the read portion of our read_buffer 
        //    and returns it to our main program
        Ok(bytes_read) => Some(read_buffer[..bytes_read].to_vec()),
        Err(_e) => None,
    }
}

// function returns the STDOUT file descriptor of the primary side of the pty
fn spawn_pty_with_shell(default_shell: String) -> RawFd {
    // forkpty is a libc function that forks the current process
    match forkpty(None, None) {
        Ok(fork_pty_res) => {
            // code on the Ok side runs in both the parent and the child process
            let stdout_fd = fork_pty_res.master;
            // we distinguish between parent and the child process by the ForkResult
            if let ForkResult::Child = fork_pty_res.fork_result {
                // I'm the secondary part of the pty
                // the child process
                // (1) we run the default shell
                Command::new(&default_shell)
                    .spawn()
                    .expect("failed to spawn");
                // (2) sleep for 2 seconds to let it load 
                std::thread::sleep(std::time::Duration::from_millis(2000));
                // (3) and then exit
                std::process::exit(0);
            }
            // in the parent process we return the stdout file descriptor to the program 
            // so that it can read what information the child sends it
            stdout_fd
        },
        Err(e) => {
            panic!("failed to fork {:?}", e);
        }
    }
}

fn main() {
    // getting the path to the system’s default shell from the SHELL environment variable
    let default_shell = std::env::var("SHELL")
        .expect("could not find default shell from $SHELL");
    
    // start it (system’s default shell) in a new process on the secondary side of a pty
    let sdtout_fd = spawn_pty_with_shell(default_shell);
    let mut read_buffer = vec![];
    loop {
        // reading from the STDOUT file descriptor
        match read_from_fd(sdtout_fd) {
            Some(mut read_bytes) => {
                read_buffer.append(&mut read_bytes);
            },
            None => {
                // we read until there’s no more data to read
                println!("{:?}", String::from_utf8(read_buffer).unwrap());
                std::process::exit(0);
            },
        }
    }
}
