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
use nix::unistd::ForkResult;
use std::os::fd::RawFd;
use std::process::Command;

// reading from the STDOUT file descriptor
fn read_from_fd(fd: RawFd) -> Option<Vec<u8>> {
    unimplemented!()
}

// function returns the STDOUT file descriptor of the primary side of the pty
fn spawn_pty_with_shell(default_shell: String) -> RawFd {
    unimplemented!()
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
