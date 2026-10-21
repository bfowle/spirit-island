//! The study screens under tmux: start `si study` in a detached session, wait for the
//! review to finish, send keys, capture the pane after each one, and compare every line
//! but the footer with a blessed fixture. Skipped when tmux, SI_DEFS_DIR or the Shroud
//! capture are absent. `SI_TUI_BLESS=1` rewrites the fixtures.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

const SOCKET: &str = "si-smoke";

fn capture_dir() -> PathBuf {
    let root = std::env::var("SI_CAPTURE_DIR").unwrap_or_else(|_| "/home/brett/repos/spirit-island/data/steam-capture".into());
    Path::new(&root).join("live-e392726411c9")
}

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/tui")
}

fn ready() -> bool {
    if Command::new("tmux").arg("-V").output().is_err() {
        eprintln!("skipping: no tmux");
        return false;
    }
    if std::env::var_os("SI_DEFS_DIR").is_none() {
        eprintln!("skipping: SI_DEFS_DIR not set");
        return false;
    }
    if !capture_dir().join("saves").is_dir() {
        eprintln!("skipping: the Shroud capture is not present");
        return false;
    }
    true
}

fn tmux(args: &[&str]) -> String {
    let out = Command::new("tmux").arg("-L").arg(SOCKET).args(args).output().expect("tmux runs");
    String::from_utf8_lossy(&out.stdout).to_string()
}

struct Session;

impl Session {
    /// `si study <args>` in a detached 220x60 session on this test's own tmux socket.
    fn start(args: &[&str]) -> Session {
        let _ = tmux(&["kill-server"]);
        let cmd = format!("{} study {}", env!("CARGO_BIN_EXE_si"), args.join(" "));
        tmux(&["new-session", "-d", "-x", "220", "-y", "60", &cmd]);
        Session
    }

    fn pane(&self) -> String {
        tmux(&["capture-pane", "-p"])
    }

    /// A tmux key name (`l`, `Enter`, `Tab`), then a beat for the redraw.
    fn key(&self, key: &str) -> String {
        tmux(&["send-keys", key]);
        std::thread::sleep(Duration::from_millis(500));
        self.pane()
    }

    fn wait_for(&self, needle: &str, timeout: Duration) -> String {
        let start = Instant::now();
        loop {
            let p = self.pane();
            if p.contains(needle) {
                std::thread::sleep(Duration::from_millis(300));
                return self.pane();
            }
            assert!(start.elapsed() < timeout, "timed out waiting for {needle:?}; pane:\n{p}");
            std::thread::sleep(Duration::from_millis(300));
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        let _ = tmux(&["kill-server"]);
    }
}

/// Every line but the footer: the last line carries timings and progress.
fn body(pane: &str) -> String {
    let lines: Vec<&str> = pane.trim_end_matches('\n').lines().collect();
    lines[..lines.len().saturating_sub(1)].iter().map(|l| l.trim_end()).collect::<Vec<_>>().join("\n")
}

fn check(case: &str, pane: &str) {
    let path = fixtures().join(format!("{case}.txt"));
    let got = body(pane);
    if std::env::var_os("SI_TUI_BLESS").is_some() {
        std::fs::create_dir_all(fixtures()).unwrap();
        std::fs::write(&path, &got).unwrap();
        return;
    }
    let want = std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("no fixture {}; run once with SI_TUI_BLESS=1", path.display()));
    if got != want {
        let diffs: Vec<String> = got
            .lines()
            .zip(want.lines())
            .enumerate()
            .filter(|(_, (a, b))| a != b)
            .take(5)
            .map(|(i, (a, b))| format!("line {}:\n  got  {a}\n  want {b}", i + 1))
            .collect();
        panic!("{case}: the pane differs from the fixture ({} vs {} lines)\n{}", got.lines().count(), want.lines().count(), diffs.join("\n"));
    }
}

#[test]
fn study_screens_on_the_shroud_capture() {
    if !ready() {
        return;
    }
    let cap = capture_dir();
    let s = Session::start(&[cap.to_str().unwrap(), "--fixed-growth"]);
    let pane = s.wait_for("turns reviewed", Duration::from_secs(180));
    check("study-turn", &pane);
    check("study-l", &s.key("l"));
    check("study-enter", &s.key("Enter"));
    check("study-e", &s.key("e"));
    check("study-pacing", &s.key("Tab"));
}
