use std::{io::Write, thread::sleep};
use sysinfo::{CpuRefreshKind, RefreshKind, System};

const EMOJI_QUESTION: &str = "❓";
const EMOJI_ZZZ: &str = "💤";
const EMOJI_GAME: &str = "🎮";
const EMOJI_FIRE: &str = "🔥";
const EMOJI_SKULL: &str = "☠️";

fn main() {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything().without_frequency()),
    );

    sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

    sys.refresh_cpu_usage();

    let cpu = sys.global_cpu_usage();
    let mut status = String::with_capacity(32);

    add_cpu_usage(cpu, &mut status);

    let mut stdout = std::io::stdout();

    stdout.write(status.as_bytes()).unwrap();
    stdout.flush().unwrap();
}

fn add_cpu_usage(usage: f32, status: &mut String) {
    let emoji = match usage as u64 {
        0..20 => EMOJI_ZZZ,
        20..60 => EMOJI_GAME,
        60..80 => EMOJI_FIRE,
        80..90 => &EMOJI_FIRE.repeat(2),
        90..99 => &EMOJI_FIRE.repeat(3),
        99..=100 => EMOJI_SKULL,
        _ => EMOJI_QUESTION,
    };

    status.push_str(emoji);
}
