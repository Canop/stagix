use {
    gix::{bstr::ByteSlice, status::index_worktree::iter::Item},
    gix_status::index_as_worktree::EntryStatus,
    std::path::PathBuf,
};

fn main() {
    // find the global gitignore location
    let conf = gix::config::File::from_globals().unwrap();
    let gitignore = conf.path_by_key("core.excludesfile").unwrap();
    println!("global gitignore: {:?}", gitignore);

    // we look either for a given path or the current directory
    let path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .or_else(|| std::env::current_dir().ok())
        .unwrap();

    // open the repository (we assume it's the root of the repository, or gix would
    // fail, but it doesn't matter for my real use case)
    let repo = gix::open(path).unwrap();

    // get head branch name
    println!(
        "head name: {:?}",
        repo.head_name().unwrap().map(|s| s.shorten().to_string())
    );

    // find all C,N,M statuses
    let progress = gix::progress::Discard {};
    let platform = repo.status(progress).unwrap();
    let iter = platform.into_index_worktree_iter(None).unwrap();
    for item in iter {
        if let Some(diff_entry) = DiffEntry::from_gix_item(item.unwrap()) {
            println!("diff entry: {:?}", diff_entry);
        }
    }
}

#[derive(Debug)]
pub enum FileStatus {
    Conflict,
    Modified,
    New,
}
#[derive(Debug)]
pub struct DiffEntry {
    pub path: PathBuf,
    pub status: FileStatus,
}

impl DiffEntry {
    pub fn from_gix_item(item: Item) -> Option<Self> {
        match item {
            Item::Modification {
                rela_path, status, ..
            } => {
                match status {
                    EntryStatus::Conflict(_) => Some(DiffEntry {
                        path: PathBuf::from(rela_path.as_bstr().to_string()),
                        status: FileStatus::Conflict,
                    }),
                    EntryStatus::Change(_) => Some(DiffEntry {
                        path: PathBuf::from(rela_path.as_bstr().to_string()),
                        status: FileStatus::Modified,
                    }),
                    EntryStatus::NeedsUpdate(_) => None, // correct ?
                    EntryStatus::IntentToAdd => None,    // correct ?
                }
            }
            Item::DirectoryContents {
                entry: gix_dir::Entry { rela_path, .. },
                ..
            } => Some(DiffEntry {
                path: PathBuf::from(rela_path.as_bstr().to_string()),
                status: FileStatus::New,
            }),
            Item::Rewrite { .. } => None,
        }
    }
}
