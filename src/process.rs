use sysinfo::{ System, Process };

fn get_processes(sys: &mut System) -> impl Iterator<Item = &Process> {
    sys.refresh_processes();

    #[cfg(windows)]
    let process_name = "東方紅魔郷.exe";
    #[cfg(unix)]
    let process_name = "東方紅魔郷";

    sys.processes_by_exact_name(process_name)
}

pub fn kill() -> bool {
    let mut sys = System::new();
    let mut ret = true;
    for process in get_processes(&mut sys) {
        if !process.kill() {
            ret = false;
        }
    }
    ret
}

pub fn is_running() -> bool {
    let mut sys = System::new();
    let is_running = match get_processes(&mut sys).next() {
        Some(_) => true,
        None => false,
    };
    is_running
}
