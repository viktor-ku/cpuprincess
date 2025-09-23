use std::thread::sleep;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

const EMOJI_CPU_1: &str = "💤";
const EMOJI_CPU_2: &str = "🎮";
const EMOJI_CPU_3: &str = "🔥";

const C_GIB_BYTES_DEL: f64 = 1_073_741_824.0;

fn main() {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything().without_frequency())
            .with_memory(MemoryRefreshKind::nothing().with_ram()),
    );

    sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

    sys.refresh_cpu_usage();
    let cpu = sys.global_cpu_usage().ceil() as u64;

    sys.refresh_memory();
    let used_bytes = sys.used_memory();

    let cpu_formatted = fmt_cpu(cpu);
    let ram_formatted = fmt_memory(used_bytes);

    println!("{}", cpu_formatted);
}

fn fmt_memory(bytes: u64) -> String {
    let gib = (bytes as f64) / C_GIB_BYTES_DEL;

    let g = gib.floor();

    format!("{} GiB", g)
}

fn fmt_cpu(usage: u64) -> String {
    let emoji = match usage {
        0..=9 => EMOJI_CPU_1,
        10..=89 => EMOJI_CPU_2,
        90..=100 => EMOJI_CPU_3,
        _ => "",
    };

    format!("{usage:2}% {emoji}")
}
