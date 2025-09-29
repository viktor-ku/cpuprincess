use std::thread::sleep;
use sysinfo::{CpuRefreshKind, RefreshKind, System};

const EMOJI_QUESTION: &str = "❓";
const EMOJI_CPU_1: &str = "💤";
const EMOJI_CPU_2: &str = "🎮";
const EMOJI_CPU_3: &str = "🔥";

fn main() {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything().without_frequency()),
    );

    sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL.mul_f32(2.0));

    sys.refresh_cpu_usage();
    let cpu = sys.global_cpu_usage().ceil() as u64;

    let cpu_formatted = fmt_cpu(cpu);

    println!("{}", cpu_formatted);
}

fn fmt_cpu(usage: u64) -> String {
    let emoji = match usage {
        0..20 => EMOJI_CPU_1,
        20..85 => EMOJI_CPU_2,
        85..=100 => EMOJI_CPU_3,
        _ => EMOJI_QUESTION,
    };

    format!("{emoji}")
}
