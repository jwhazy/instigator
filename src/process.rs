use std::path::PathBuf;
use std::process::Command;

use log::{error, info};
use winapi::shared::minwindef::FALSE;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::{OpenThread, SuspendThread};
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Thread32First, Thread32Next, TH32CS_SNAPTHREAD, THREADENTRY32,
};
use winapi::um::winnt::HANDLE;
use winapi::um::winnt::THREAD_SUSPEND_RESUME;

use sysinfo::System;

pub fn kill_fortnite() {
    let mut system = System::new_all();
    system.refresh_all();

    for ac in system.processes_by_name("FortniteClient-Win64-Shipping.exe") {
        info!("Killing process: {}", ac.name());
        ac.kill();
    }
}

pub fn kill_all() {
    let mut system = System::new_all();
    system.refresh_all();

    for launcher in system.processes_by_name("FortniteLauncher.exe") {
        info!("Killing process: {}", launcher.name());
        launcher.kill();
    }

    for ac in system.processes_by_name("FortniteClient-Win64-Shipping_EAC.exe") {
        info!("Killing process: {}", ac.name());
        ac.kill();
    }
}

pub fn start_ac(path: &PathBuf) {
    let mut ac_path = PathBuf::from(&path);
    ac_path.push("FortniteGame\\Binaries\\Win64\\FortniteClient-Win64-Shipping_EAC.exe");

    let mut cwd = PathBuf::from(&path);
    cwd.push("FortniteGame\\Binaries\\Win64");

    let process = Command::new(ac_path).current_dir(&cwd).spawn();

    match process {
        Ok(result) => suspend_process(result.id()),
        Err(_err) => {
            error!("Could not start EAC process. Make sure it exists before trying again.");
            (0, false)
        }
    };
}

pub fn start_launcher(path: &PathBuf) {
    let mut launcher_path = PathBuf::from(&path);
    launcher_path.push("FortniteGame\\Binaries\\Win64\\FortniteLauncher.exe");

    let mut cwd = PathBuf::from(&path);
    cwd.push("FortniteGame\\Binaries\\Win64");

    let process = Command::new(launcher_path).current_dir(&cwd).spawn();

    match process {
        Ok(result) => suspend_process(result.id()),
        Err(_err) => {
            error!("Could not start FortniteLauncher process. Make sure it exists before trying again.");
            (0, false)
        }
    };
}

// Credit: afc11hn.  https://www.reddit.com/r/rust/comments/xu2hiw/comment/iqtrpb5
pub fn suspend_process(pid: u32) -> (u32, bool) {
    unsafe {
        let mut has_err = false;
        let mut count: u32 = 0;

        let te: &mut THREADENTRY32 = &mut std::mem::zeroed();
        (*te).dwSize = std::mem::size_of::<THREADENTRY32>() as u32;

        let snapshot: HANDLE = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);

        if Thread32First(snapshot, te) == 1 {
            loop {
                if pid == (*te).th32OwnerProcessID {
                    let tid = (*te).th32ThreadID;

                    let thread: HANDLE = OpenThread(THREAD_SUSPEND_RESUME, FALSE, tid);
                    has_err |= SuspendThread(thread) as i32 == -1i32;

                    CloseHandle(thread);
                    count += 1;
                }

                if Thread32Next(snapshot, te) == 0 {
                    break;
                }
            }
        }

        CloseHandle(snapshot);

        return (count, has_err);
    }
}
