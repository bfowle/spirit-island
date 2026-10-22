//! The live recorder's journal stream across screen restarts: the stream keeps one copy
//! of what the app wrote, however often a screen starts over the same game.

use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use si_steam::source::Recorder;

fn scratch(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("si-recorder-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(d.join("steam")).unwrap();
    d
}

/// The stream's lines without their stamps.
fn stream(data: &Path) -> Vec<String> {
    std::fs::read_to_string(data.join("steam-capture/live-g1/log-stream.txt")).unwrap_or_default().lines().map(|l| l[14..].to_string()).collect()
}

/// A screen starting over the folder: the whole journal is read, then the first save names the game.
fn screen(data: &Path, log: &Path) {
    let mut r = Recorder::new(data);
    r.tail(log);
    r.set_game("g1");
}

#[test]
fn a_restarted_screen_adds_no_second_copy_of_the_journal() {
    let d = scratch("restart");
    let log = d.join("steam/log.txt");
    std::fs::write(&log, "GameView Start\nline a\nline b\n").unwrap();
    screen(&d, &log);
    assert_eq!(stream(&d), ["GameView Start", "line a", "line b"]);
    // the screen restarts over the same game and reads the same file again
    screen(&d, &log);
    assert_eq!(stream(&d), ["GameView Start", "line a", "line b"], "no second copy");
    // the app wrote on while no screen ran: only the new line is appended
    std::fs::write(&log, "GameView Start\nline a\nline b\nline c\n").unwrap();
    screen(&d, &log);
    assert_eq!(stream(&d), ["GameView Start", "line a", "line b", "line c"]);
    // the app started its journal afresh (a shorter file): a new copy follows the old
    std::fs::write(&log, "GameView Start\nline z\n").unwrap();
    screen(&d, &log);
    assert_eq!(stream(&d), ["GameView Start", "line a", "line b", "line c", "GameView Start", "line z"]);
    let _ = std::fs::remove_dir_all(&d);
}

#[test]
fn the_same_content_written_again_later_is_a_new_copy() {
    let d = scratch("rewrite");
    let log = d.join("steam/log.txt");
    std::fs::write(&log, "GameView Start\n***** LOADED CONTINUE FILE *****\n").unwrap();
    screen(&d, &log);
    assert_eq!(stream(&d).len(), 2);
    // the app loaded again and wrote the identical opening after the stream's last line
    let f = std::fs::OpenOptions::new().write(true).open(&log).unwrap();
    f.set_modified(SystemTime::now() + Duration::from_secs(10)).unwrap();
    screen(&d, &log);
    assert_eq!(stream(&d).len(), 4, "a rewritten journal is a new copy, even when its text is the same");
    let _ = std::fs::remove_dir_all(&d);
}

/// Unity's Player.log restarts with the app, not with the game: its stream keeps one
/// copy per launch, and a relaunch does not unname the game.
#[test]
fn the_player_log_is_streamed_beside_the_journal() {
    let d = scratch("player");
    let log = d.join("steam/log.txt");
    let player = d.join("steam/Player.log");
    std::fs::write(&log, "GameView Start\nline a\n").unwrap();
    std::fs::write(&player, "Unity boot\nHandleWillPerformAction start: [A]\n").unwrap();
    let mut r = Recorder::new(&d);
    r.tail(&log);
    r.tail_player(&player);
    r.set_game("g1");
    let pstream = |d: &Path| std::fs::read_to_string(d.join("steam-capture/live-g1/player-stream.txt")).unwrap_or_default().lines().map(|l| l[14..].to_string()).collect::<Vec<_>>();
    assert_eq!(pstream(&d), ["Unity boot", "HandleWillPerformAction start: [A]"]);
    // more lines while the screen runs
    std::fs::write(&player, "Unity boot\nHandleWillPerformAction start: [A]\nHandleDidPerformAction start: [A]\n").unwrap();
    r.tail_player(&player);
    assert_eq!(pstream(&d).len(), 3);
    // the app relaunches (a shorter Player.log): a new copy, the game still named
    std::fs::write(&player, "Unity boot\n").unwrap();
    r.tail_player(&player);
    assert_eq!(r.game(), Some("g1"));
    assert_eq!(pstream(&d), ["Unity boot", "HandleWillPerformAction start: [A]", "HandleDidPerformAction start: [A]", "Unity boot"]);
    // a screen restarted over the same game adds no second copy of either stream
    screen(&d, &log);
    let mut r2 = Recorder::new(&d);
    r2.tail_player(&player);
    r2.set_game("g1");
    assert_eq!(pstream(&d).len(), 4);
    assert_eq!(stream(&d), ["GameView Start", "line a"]);
    let _ = std::fs::remove_dir_all(&d);
}
