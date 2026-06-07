use scx_rustland_core::Scheduler;
use std::mem::MaybeUninit;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inicjalizacja podsystemu logowania
    env_logger::init();
    println!("[INFO] Starting scx_throughput CPU scheduler...");

    // Alokacja pamięci dla obiektu eBPF
    let mut open_object = MaybeUninit::uninit();

    // Rejestracja planisty w podsystemie sched-ext jądra Linux
    let mut sched = match Scheduler::init(&mut open_object) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[ERROR] Failed to load scheduler into the kernel: {}", e);
            eprintln!("[FATAL] Please ensure the process is running with root privileges (sudo).");
            return Err(e.into());
        }
    };

    println!("[INFO] Scheduler successfully attached to sched-ext.");
    println!("[INFO] Press Ctrl+C to terminate and restore default EEVDF scheduler.");

    // Główna pętla wykonawcza planisty
    loop {
        if !sched.run()?.should_restart() {
            break;
        }
    }

    println!("[INFO] Termination requested. Restoring default CPU scheduling architecture.");
    Ok(())
}