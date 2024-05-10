

This project is a study in how to achieve some "git status" related tasks using [gix](https://docs.rs/gix/latest/gix/), a library in an early development stage whose main advantage is in being full rust.

The end goal for me is to replace git2 in several programs, as git2 brings a lot of build problems and limits compatibility.

Here are the needed tasks and progress:
* [x] find the global gitignore location
* [x] open the repository (we assume it's the root of the repository, or gix would
* [x] get head branch name
* [x] find the C,N,M statuses of work files compared to head
* [ ] get a +/- summary of lines added removed in work dir (*doesn't seem possible atm*)

This simple executable is ran either in the git repository or given its path:

     cargo run -- ~/dev/broot
