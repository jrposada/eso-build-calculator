use colored::Colorize;
use std::io::{self, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

static LAST_PROGRESS_LINE_COUNT: AtomicUsize = AtomicUsize::new(0);
static LAST_WAS_PROGRESS: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static STDOUT_LOCK: Mutex<()> = Mutex::new(());

fn clear_multiline_progress() {
    let line_count = LAST_PROGRESS_LINE_COUNT.load(Ordering::SeqCst);
    if line_count > 0 {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        for _ in 0..line_count {
            let _ = write!(handle, "\x1b[A\x1b[K");
        }
        let _ = handle.flush();
        LAST_PROGRESS_LINE_COUNT.store(0, Ordering::SeqCst);
    }
}

fn clear_progress() {
    clear_multiline_progress();
    if LAST_WAS_PROGRESS.load(Ordering::SeqCst) {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        let _ = write!(handle, "\r\x1b[K");
        let _ = handle.flush();
        LAST_WAS_PROGRESS.store(false, Ordering::SeqCst);
    }
}

/// Log a plain message
pub fn log(message: &str) {
    let _lock = STDOUT_LOCK.lock().unwrap();
    clear_progress();
    println!("{}", message);
}

/// Log an info message (blue)
pub fn info(message: &str) {
    let _lock = STDOUT_LOCK.lock().unwrap();
    clear_progress();
    println!("{}", message.blue());
}

/// Log a warning message (yellow)
pub fn warn(message: &str) {
    let _lock = STDOUT_LOCK.lock().unwrap();
    clear_progress();
    eprintln!("{}", message.yellow());
}

/// Log an error message (red)
pub fn error(message: &str) {
    let _lock = STDOUT_LOCK.lock().unwrap();
    clear_progress();
    eprintln!("{}", message.red());
}

/// Log a success message (green)
pub fn success(message: &str) {
    let _lock = STDOUT_LOCK.lock().unwrap();
    clear_progress();
    println!("{}", message.green());
}

/// Log a dim message (gray)
pub fn dim(message: &str) {
    let _lock = STDOUT_LOCK.lock().unwrap();
    clear_progress();
    println!("{}", message.dimmed());
}

/// Log a progress message (overwrites current line)
pub fn progress(message: &str) {
    let _lock = STDOUT_LOCK.lock().unwrap();
    clear_multiline_progress();
    if LAST_WAS_PROGRESS.load(Ordering::SeqCst) {
        print!("\r\x1b[K");
    }
    print!("{}", message.dimmed());
    let _ = io::stdout().flush();
    LAST_WAS_PROGRESS.store(true, Ordering::SeqCst);
}

/// Log a multiline progress message (can be cleared)
pub fn progress_multiline(message: &str) {
    let _lock = STDOUT_LOCK.lock().unwrap();
    clear_progress();
    let line_count = message.lines().count();
    println!("{}", message.dimmed());
    LAST_PROGRESS_LINE_COUNT.store(line_count, Ordering::SeqCst);
}

/// Format a duration in milliseconds to human-readable string
pub fn format_duration(ms: u64) -> String {
    let seconds = ms / 1000;
    let minutes = seconds / 60;
    let hours = minutes / 60;

    if hours > 0 {
        format!("{}h {}m", hours, minutes % 60)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds % 60)
    } else {
        format!("{}s", seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(500), "0s");
        assert_eq!(format_duration(1000), "1s");
        assert_eq!(format_duration(65000), "1m 5s");
        assert_eq!(format_duration(3665000), "1h 1m");
    }
}
