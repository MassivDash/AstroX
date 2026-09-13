use crate::cli::utils::terminal::{dev_info, do_front_log, do_server_log, success};
use std::io::{BufRead, BufReader, Read};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::sleep;
use std::time::Duration;

/// Read `reader` line by line and hand every non empty line to `on_line`.
/// Trailing `\r` / `\n` are stripped, invalid utf8 is replaced.
pub fn stream_lines<R: Read>(reader: R, mut on_line: impl FnMut(&str)) {
    let mut reader = BufReader::new(reader);
    let mut buf = Vec::new();

    while let Ok(bytes_read) = reader.read_until(b'\n', &mut buf) {
        if bytes_read == 0 {
            break;
        }
        let line = String::from_utf8_lossy(&buf)
            .trim_end_matches(&['\r', '\n'][..])
            .to_string();
        buf.clear();
        if !line.trim().is_empty() {
            on_line(&line);
        }
    }
}

/// Block until the `ready` flag has been raised by a log reading thread.
pub fn wait_until_ready(ready: &AtomicBool) {
    while !ready.load(Ordering::SeqCst) {
        sleep(Duration::from_millis(100));
    }
}

/// Does this actix log line mean the backend is up ?
pub fn is_actix_ready_line(line: &str) -> bool {
    line.contains("Actix server has started 🚀")
}

/// Does this astro log line mean the frontend is up ?
pub fn is_astro_ready_line(line: &str) -> bool {
    line.contains("ready")
}

/// Log an actix line and raise `ready` the first time the backend reports being up.
pub fn handle_actix_line(line: &str, host: &str, port: u16, ready: &AtomicBool) {
    do_server_log(&format!("{}\n", line));

    if !ready.load(Ordering::SeqCst) && is_actix_ready_line(line) {
        ready.store(true, Ordering::SeqCst);
        dev_info(host, port);
        success("Actix server is running, starting the frontend development server");
    }
}

/// Opens a url in the user's browser.
pub trait BrowserOpener {
    fn open(&self, url: &str);
}

pub struct RealBrowserOpener;

impl BrowserOpener for RealBrowserOpener {
    fn open(&self, url: &str) {
        let browser = Command::new("open").arg(url).spawn();

        if let Err(err) = browser {
            println!("Failed to execute command: {}", err);
            println!("Are You a Ci Secret Agent ?");
        }
    }
}

/// Log an astro line and, the first time the frontend reports being ready,
/// open the browser on the astro development server.
pub fn handle_astro_line(
    line: &str,
    astro_port: u16,
    ready: &AtomicBool,
    opener: &dyn BrowserOpener,
) {
    do_front_log(&format!("{}\n", line));

    if !ready.load(Ordering::SeqCst) && is_astro_ready_line(line) {
        ready.store(true, Ordering::SeqCst);
        success("Astro is ready, opening the browser");
        opener.open(&format!("http://localhost:{}", astro_port));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::io::Cursor;

    struct SpyBrowserOpener {
        opened: RefCell<Vec<String>>,
    }

    impl SpyBrowserOpener {
        fn new() -> Self {
            Self {
                opened: RefCell::new(vec![]),
            }
        }
    }

    impl BrowserOpener for SpyBrowserOpener {
        fn open(&self, url: &str) {
            self.opened.borrow_mut().push(url.to_string());
        }
    }

    #[test]
    fn test_stream_lines_skips_blank_lines_and_trims_line_endings() {
        let input = Cursor::new("first\r\n\n   \nsecond\nthird".as_bytes());
        let mut lines: Vec<String> = vec![];

        stream_lines(input, |line| lines.push(line.to_string()));

        assert_eq!(lines, vec!["first", "second", "third"]);
    }

    #[test]
    fn test_stream_lines_on_empty_reader() {
        let mut calls = 0;
        stream_lines(Cursor::new(Vec::new()), |_| calls += 1);
        assert_eq!(calls, 0);
    }

    #[test]
    fn test_stream_lines_replaces_invalid_utf8() {
        let input = Cursor::new(vec![0xff, b'o', b'k', b'\n']);
        let mut lines: Vec<String> = vec![];

        stream_lines(input, |line| lines.push(line.to_string()));

        assert_eq!(lines.len(), 1);
        assert!(lines[0].ends_with("ok"));
    }

    #[test]
    fn test_wait_until_ready_returns_when_the_flag_is_already_up() {
        let ready = AtomicBool::new(true);
        wait_until_ready(&ready);
        assert!(ready.load(Ordering::SeqCst));
    }

    #[test]
    fn test_is_actix_ready_line() {
        assert!(is_actix_ready_line("Actix server has started 🚀"));
        assert!(!is_actix_ready_line("Actix server has started"));
        assert!(!is_actix_ready_line("compiling backend"));
    }

    #[test]
    fn test_is_astro_ready_line() {
        assert!(is_astro_ready_line("astro is ready in 300ms"));
        assert!(!is_astro_ready_line("building"));
    }

    #[test]
    fn test_handle_actix_line_raises_ready() {
        let ready = AtomicBool::new(false);

        handle_actix_line("compiling", "localhost", 8080, &ready);
        assert!(!ready.load(Ordering::SeqCst));

        handle_actix_line("Actix server has started 🚀", "localhost", 8080, &ready);
        assert!(ready.load(Ordering::SeqCst));
    }

    #[test]
    fn test_handle_astro_line_opens_the_browser_once() {
        let ready = AtomicBool::new(false);
        let opener = SpyBrowserOpener::new();

        handle_astro_line("building", 4321, &ready, &opener);
        assert!(!ready.load(Ordering::SeqCst));
        assert!(opener.opened.borrow().is_empty());

        handle_astro_line("astro ready", 4321, &ready, &opener);
        assert!(ready.load(Ordering::SeqCst));
        assert_eq!(*opener.opened.borrow(), vec!["http://localhost:4321"]);

        // Already ready, the browser must not be opened a second time
        handle_astro_line("astro ready", 4321, &ready, &opener);
        assert_eq!(opener.opened.borrow().len(), 1);
    }
}
