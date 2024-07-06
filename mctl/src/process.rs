use std::{io, ffi::OsString, fs, fs::DirEntry, os::linux::fs::MetadataExt, process};

const BIN_NAME: &str = env!("CARGO_BIN_NAME");

extern "C" {
    fn geteuid() -> u32;
}

struct ProcInfo {
    pid: OsString,
    uid: u32,
}

impl ProcInfo {
    fn new() -> Self {
        Self {
            uid: unsafe { geteuid() },
            pid: OsString::from(process::id().to_string()),
        }
    }
}

fn is_other_mctl_proc_dir(dir: io::Result<DirEntry>, proc: &ProcInfo) -> Option<DirEntry> {
    dir.ok()
        .filter(|dir| dir.path().file_name().map_or(false, |p| p != proc.pid))
        .and_then(|dir| {
            dir.metadata()
                .ok()
                .filter(|meta| meta.is_dir() && meta.st_uid() == proc.uid)
                .map(|_| dir)
        })
        .and_then(|dir| {
            let mut path = dir.path();
            path.push("comm");

            fs::read_to_string(path)
                .ok()
                .filter(|contents| contents.trim() == BIN_NAME)
                .map(|_| dir)
        })
}

fn to_pid(dir: DirEntry) -> u32 {
    dir.path()
        .file_name()
        .expect("/proc/<pid>: could not find dir name")
        .to_str()
        .expect("/proc/<pid>: could not convert to string")
        .parse()
        .expect("/proc/<pid>: could not parse pid as integer")
}

pub fn kill_other_mctl_processes() {
    let info = ProcInfo::new();

    fs::read_dir("/proc")
        .expect("Could not read /proc")
        .filter_map(|dir| is_other_mctl_proc_dir(dir, &info))
        .map(&to_pid)
        .for_each(|pid| {
            log::info!("Stopping existing mctl process {}", pid);
            unsafe { libc::kill(pid as i32, libc::SIGTERM) };
        })
}
