use scx_rustland_core::RustLandBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    println!("[INFO] Starting scx_throughput...");

    // RustLandBuilder skompiluje i załaduje kod eBPF
    let mut builder = RustLandBuilder::new()?;
    builder.build()?;

    println!("[INFO] Scheduler successfully attached.");
    
    // Pętla podtrzymująca życie procesu
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}