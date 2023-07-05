use std::path::PathBuf;

use futures::channel::mpsc::channel;
use futures::{SinkExt, StreamExt};
use notify::event::{DataChange, ModifyKind};
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};

use crate::update::UpdateType;

#[derive(Debug)]
pub enum WatchEvent {
    Modified(Vec<PathBuf>),
    #[allow(dead_code)]
    Removed(Vec<PathBuf>),
}

impl From<WatchEvent> for Vec<(PathBuf, UpdateType)> {
    fn from(event: WatchEvent) -> Self {
        match event {
            WatchEvent::Modified(paths) => paths
                .into_iter()
                .map(|path| (path, UpdateType::Modify))
                .collect(),
            WatchEvent::Removed(paths) => paths
                .into_iter()
                .map(|path| (path, UpdateType::Remove))
                .collect(),
        }
    }
}

pub fn watch<T>(root: &PathBuf, func: T)
where
    T: FnMut(WatchEvent),
{
    futures::executor::block_on(async {
        watch_async(root, func).await;
    });
}

pub async fn watch_async<T>(root: &PathBuf, mut func: T)
where
    T: FnMut(WatchEvent),
{
    let (mut tx, mut rx) = channel(2);
    let mut watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        notify::Config::default(),
    )
    .unwrap();

    watcher.watch(root, RecursiveMode::NonRecursive).unwrap();

    std::fs::read_dir(root).unwrap().for_each(|entry| {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            watcher
                .watch(path.as_path(), RecursiveMode::NonRecursive)
                .unwrap();
        } else {
            // TODO respect to .gitignore sth like that
            let path_str = path.to_string_lossy();
            if path_str.contains("node_modules")
                || path_str.contains(".git")
                || path_str.contains("dist")
            {
                return;
            }
            watcher
                .watch(path.as_path(), RecursiveMode::Recursive)
                .unwrap();
        }
    });

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => match event.kind {
                EventKind::Any => {}
                EventKind::Access(_) => {}
                EventKind::Create(_) => {
                    // a new create file trigger both Create and Modify Event
                }
                EventKind::Modify(ModifyKind::Data(DataChange::Any)) => {
                    println!("{:?}", event);
                    func(crate::watch::WatchEvent::Modified(event.paths));
                }
                EventKind::Modify(_) => {}
                EventKind::Remove(_) => {
                    println!("{:?}", event);
                }
                EventKind::Other => {}
            },
            Err(e) => {
                println!("watch error: {:?}", e);
            }
        }
    }
}
