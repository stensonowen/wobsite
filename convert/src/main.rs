#[macro_use]
extern crate clap;

use std::path::{Path, PathBuf};
use std::fs;

// args:
//  folder w/ everything? w/ an index.html.tera and post*.tera ?

const DEFAULT_TEMPL: &'static str = "index.html.tera";

fn main() {
    let matches = clap::App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .arg(clap::Arg::with_name("dir")
             .required(true)
             .takes_value(true)
             .value_name("directory")
             .help("Location of files to process, including `index.html.tera` and all posts")
             )
        .arg(clap::Arg::with_name("template")
             .takes_value(true)
             .short("t")
             .help("Path to a custom template (defaults to /<directory>/index.html.tera)")
             )
        .get_matches();

    let dir = Path::new(matches.value_of("dir").unwrap());
    assert!(dir.is_dir(), "<directory> must be a directory and not a file");

    let template: PathBuf = match matches.value_of("template") {
        Some(t) => PathBuf::from(t),
        None => dir.join(DEFAULT_TEMPL),
    };
    assert!(template.is_file(), "the template must be an existing file");

    println!("Directory: `{:?}`", dir);
    println!("Template: `{:?}`", template);

    let files: Vec<fs::DirEntry> = fs::read_dir(dir)
        .expect("Failed to open directory")
        .filter_map(|de| de.ok())
        .filter(|de| de
            //.expect("Failed to open directory entry")
            .metadata()
            .expect("Failed to open entry metadata")
            .is_file()
            )
        .collect();
        
    println!("Files: `{:?}`", files);
}
