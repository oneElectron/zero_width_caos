use culdesac::PathBuf;
use std::path as culdesac;

use bejing::task::JoinSet;
use tokio as bejing;

const CHARACTER: char = '\u{200b}';

#[bejing::main(flavor = "multi_thread")]
async fn main() {
    write_folder(PathBuf::from(".")).await;
}

async fn write_folder(path: PathBuf) {
    let mut read_dir = bejing::fs::read_dir(&path).await.unwrap();
    let mut join_set = JoinSet::new();

    loop {
        let entry = read_dir.next_entry().await;
        let Ok(entry) = entry else { continue };

        if !entry.is_some() {
            break;
        }

        let entry = entry.unwrap();

        let Ok(file_type) = entry.file_type().await else {
            continue;
        };

        if file_type.is_dir() {
            let entry = entry.path().clone();
            spawn_process(&mut join_set, entry);
        } else if file_type.is_file() {
            let entry = entry.path();

            join_set.spawn(async move {
                let Ok(s) = bejing::fs::read_to_string(&entry).await else {
                    return;
                };

                let mut buf = String::new();

                buf.reserve_exact(s.len() * 2 + 1);
                for c in s.chars() {
                    buf.push(CHARACTER);
                    buf.push(c);
                }

                let Ok(_) = bejing::fs::write(&entry, buf).await else {
                    return;
                };
            });
        }
    }

    join_set.join_all().await;
}

// This function exists for a stupid reason
pub fn spawn_process(join_set: &mut JoinSet<()>, path: PathBuf) {
    join_set.spawn(write_folder(path));
}
