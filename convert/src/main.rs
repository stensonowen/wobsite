#[macro_use]
extern crate clap;
extern crate tera;

use std::path::{Path, PathBuf};
use std::fs;
use std::io::{BufRead, BufReader};
use std::ffi::OsStr;

// args:
//  folder w/ everything? w/ an index.html.tera and post*.tera ?

const DEFAULT_TEMPL: &'static str = "template.html.tera";
static POST_SUFFIX: &'static str = ".post";

fn main() {
    //TODO: modify index.html
    let matches = clap::App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .arg(clap::Arg::with_name("dir")
             .required(true)
             .takes_value(true)
             .value_name("directory")
             .help("Location of files to process (`index`/`template.html.tera` and posts)")
             )
        .arg(clap::Arg::with_name("template")
             .takes_value(true)
             .short("t")
             .help("Path to a custom template (defaults to /<directory>/template.html.tera)")
             )
        .get_matches();

    //dir: path where all content (and maybe the template) is located
    let dir = Path::new(matches.value_of("dir").unwrap());
    assert!(dir.is_dir(), "<directory> must be a directory and not a file");

    //what we're going to feed all the content in `dir` into
    let template: PathBuf = match matches.value_of("template") {
        Some(t) => PathBuf::from(t),
        None => dir.join(DEFAULT_TEMPL),
    };
    assert!(template.is_file(), "the template must be an existing file");

    //all the actual content we'll be using
    // follow directories recursively?
    // ignore files we can't get a handle to (complain explicitly?)
    let post_type: Option<&OsStr> = Some(OsStr::new(POST_SUFFIX));
    let files: Vec<fs::DirEntry> = fs::read_dir(dir)
        .expect("Failed to open directory")
        .filter_map(|de| de.ok())
        .filter(|de| de
                .path().extension() == post_type
            //.metadata()
            //.expect("Failed to open entry metadata")
            //.is_file()  // filter by extension instead?
            )
        .collect();
        
    println!("Files: `{:?}`", files);

    //read file data; construct content
    //see Cobalt.rs for post layout (slightly added onto markdown)
    // https://github.com/cobalt-org/cobalt.rs#posts

    //unwrap shouldn't be a problem; do we need a graceful exit for bad utf8?
    //let template = tera::Tera::new(template.to_str().unwrap()).unwrap(); 
    let template = tera::Tera::new("*.html.tera").unwrap();
    let mut context = tera::Context::new();

    //only iterate through `.post` files
    for path in files {
        println!("Opening file: `{:?}`", path);
        let mut s = String::new();
        let f = fs::File::open(path.path()).expect("Unable to open file");
        let mut r = BufReader::new(f);
        println!("{:?}", parse_header(&mut r));

    }

}


fn parse_header(br: &mut BufReader<fs::File>) -> Result<tera::Context,ParseError> {
    let mut context = tera::Context::new();
    let mut s = String::new();
    let mut count = 0;
    loop {
        s.clear();
        let r = br.read_line(&mut s);
        if r.is_err() || r.unwrap() == 0 {  // premature death
            return Err(ParseError::Unfinished);
        } else if s.starts_with("---") {    // done
            return Ok(context);
        }

        let mid: usize = s.find(':').ok_or(ParseError::InvalidLine(count))?;
        let (key, value) = s.split_at(mid);
        println!("First half: `{}`;  second half: `{}`", key, value);
        //let line = s.trim();

        count += 1;
    }
}

#[derive(Debug)]
enum ParseError {
    Unfinished,
    InvalidLine(usize),
}
