use std::{path::PathBuf, thread, sync::{Arc, Mutex}};
use ignore::WalkBuilder;

pub fn search_file_in_dir(query: &str, dir: &str) -> std::thread::JoinHandle<Vec<PathBuf>> {
    let query = query.to_string();
    let dir = dir.to_string();

    thread::spawn(move || {
        let files = Arc::new(Mutex::new(Vec::new()));

        let walker = WalkBuilder::new(dir)
            .hidden(false)
            .ignore(false)
            .git_ignore(false)
            .build_parallel();

        walker.run(|| {
            let query = query.clone();
            let files = Arc::clone(&files);

            Box::new(move |result| {
                if let Ok(entry) = result {
                    if let Some(name) = entry.file_name().to_str() {
                        if name.contains(&query) {
                            files.lock().unwrap().push(entry.into_path());
                        }
                    }
                }
                ignore::WalkState::Continue
            })
        });

        Arc::try_unwrap(files).unwrap().into_inner().unwrap()
    })
}
