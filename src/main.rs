use culdesac::PathBuf as B;
use sdt::path as culdesac;
use std as sdt;

use bejing::task::JoinSet as A;
use tokio as bejing;

const F: char = '\u{200b}';

macro_rules! E {
    ($g: ident, $u: ident, $h: tt) => {
        let Ok($g) = $u else { $h };
    };
    () => {
        B::from(".")
    };
}

#[bejing::main(flavor = "multi_thread")]
async fn main() {
    e(E!()).await;
}

async fn e(q: B) {
    let mut r = bejing::fs::read_dir(&q).await.unwrap();
    let mut i = A::new();
    loop {
        let d = r.next_entry().await;
        E!(l, d, continue);
        if l.is_some() {
            let g = l.unwrap();
            if let Ok(t) = g.file_type().await {
                if t.is_dir() {
                    let g = g.path().clone();
                    mod e {
                        // This function exists for a stupid reason
                        pub fn p(l: &mut super::A<()>, b: super::B) {
                            l.spawn(crate::e(b));
                        }
                    }
                    e::p(&mut i, g);
                } else if t.is_file() {
                    let g = g.path();
                    i.spawn(async move {
                        if let Ok(s) = bejing::fs::read_to_string(&g).await {
                            let mut w = String::new();
                            w.reserve_exact(s.len() * 2 + 1);
                            for c in s.chars() {
                                w.push(F);
                                w.push(c);
                            }
                            let Ok(_) = bejing::fs::write(&g, w).await else {
                                return;
                            };
                        } else {
                            return;
                        };
                    });
                }
            }
        } else {
            break;
        };
    }
    i.join_all().await;
}
