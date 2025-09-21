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

    sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL.mul_f64(1.3));

    sys.refresh_cpu_usage();
    sys.refresh_memory();

    let cpu = sys.global_cpu_usage().ceil() as u64;
    let used_mem = (sys.used_memory() as f64) / C_GIB_BYTES_DEL;

    let trunc = used_mem.trunc() as u64;
    let fract = (used_mem.fract() * 100.0).round() as u64;

    let emoji = match cpu {
        0..=9 => EMOJI_CPU_1,
        10..=89 => EMOJI_CPU_2,
        90..=100 => EMOJI_CPU_3,
        _ => "",
    };

    println!("{trunc}.{fract} GiB {emoji} {cpu}%");
}
