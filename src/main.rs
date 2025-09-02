use std::thread::sleep;
use sysinfo::{CpuRefreshKind, RefreshKind, System};

fn main() {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything().without_frequency()),
    );

    sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

    sys.refresh_cpu_usage();

    let cpu = sys.global_cpu_usage().ceil() as u64;

    let emoji = match cpu {
        0..=9 => "💤",
        10..=89 => "🎮",
        90..=100 => "🔥",
        _ => " ",
    };

    println!("CPU {}% {}", cpu, emoji);
}
