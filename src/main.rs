use nix::unistd::Pid;
use std::os::unix::process::CommandExt;

fn main() {
    let mut child = unsafe {
        std::process::Command::new("python3")
            .pre_exec(|| {
                let pid = nix::unistd::getpid();
                nix::unistd::setpgid(pid, pid).expect("could not set group id");
                Ok(())
            })
            .spawn()
            .unwrap()
    };

    let child_pid = child.id();
    nix::unistd::tcsetpgrp(0, Pid::from_raw(child_pid as i32))
        .expect("could not set terminal group id");

    child.wait().unwrap();

    nix::unistd::tcsetpgrp(0, nix::unistd::getpid()).expect("could not set terminal group id");
}
