use std::path::PathBuf;

pub fn data_dir() -> PathBuf {
    let dir = dirs_or_home().join(".typiano");
    std::fs::create_dir_all(&dir).ok();
    dir
}

pub fn socket_path() -> PathBuf {
    data_dir().join("typiano.sock")
}

pub fn pid_path() -> PathBuf {
    data_dir().join("typiano.pid")
}

pub fn user_songs_dir() -> PathBuf {
    let dir = data_dir().join("songs");
    std::fs::create_dir_all(&dir).ok();
    dir
}

fn dirs_or_home() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
}
