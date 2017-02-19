#[macro_use]
extern crate clap;
extern crate tera;

use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, BufRead, BufReader, Write, Read};
use std::ffi::OsStr;
use tera::Tera;

//see Cobalt.rs for post layout (slightly added onto markdown)
// https://github.com/cobalt-org/cobalt.rs#posts

// args:
//  folder w/ everything? w/ an index.html.tera and post*.tera ?

// will look for *.html.tera
const DEFAULT_TEMPL: &'static str = "template.html.tera";
static POST_SUFFIX: &'static str = "post";

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
    let template_path: PathBuf = match matches.value_of("template") {
        Some(t) => PathBuf::from(t),
        None => dir.join(DEFAULT_TEMPL),
    };
    let template_name_s = template_path.file_name().unwrap().to_str().unwrap();

    assert!(template_path.is_file(), "the template must be an existing file");

    //all the actual content we'll be using
    // follow directories recursively?
    // ignore files we can't get a handle to (complain explicitly?)
    let post_type: Option<&OsStr> = Some(OsStr::new(POST_SUFFIX));
    let files: Vec<PathBuf> = fs::read_dir(dir)
        .expect("Failed to open directory")
        .filter_map(|de| match de {
            Ok(d) => Some(d.path()),
            Err(_) => None,
        })
        .filter(|pb| pb .extension() == post_type)
        .collect();
        
    println!("Files: `{:?}`", files);

    //unwrap shouldn't be a problem; do we need a graceful exit for bad utf8?
    let mut all_templates = template_path.clone();
    all_templates.set_file_name("*.html.tera");
    let all_templates_s = rel_str_from_pathbuf(&all_templates);
    let templates = tera::Tera::new(all_templates_s).unwrap();

    //only iterate through `.post` files
    for path in files {
        println!("Opening file: `{:?}`", path);
        //tera doesn't like 
        let page_path = dir.join(path.file_stem().unwrap()).with_extension("html");
        let page = render_page(&templates, template_name_s, &path);
        save_page(page, &page_path)
            .expect(&format!("Failed to save flie {:?}", page_path));
        println!("Saved file to `{:?}`", page_path);
    }

}


fn rel_str_from_pathbuf<'a>(pb: &'a Path) -> &'a str { //bonus lifetime
    if pb.starts_with("./") {
        pb.file_name().unwrap()
    } else {
        pb.as_os_str()
    }.to_str().unwrap()
}


fn save_page(page: String, dest: &Path) -> Result<(), io::Error>{
    //writes page to destination
    //panics if weird os issue
    let mut f = fs::File::create(dest)?;
    f.write_all(page.as_bytes())
}

fn render_page(templates: &Tera, template: &str, path: &PathBuf) -> String {
    //can throw errors if template/context pair invalid: non-recoverable
    let f = fs::File::open(path).expect("Unable to open file");
    let mut r = BufReader::new(f);
    let context = parse_content(&mut r).expect("Failed to parse file config");
    templates.render(template, context)
        .unwrap_or_else(|e| 
            panic!(format!("Failed to apply context {:?}\n\tto template {:?}:\n\t{:?}", 
                           path, template, e)))
}


fn parse_content(br: &mut BufReader<fs::File>) -> Result<tera::Context,ParseError> {
    let mut context = tera::Context::new();
    let mut s = String::new();
    let mut count = 0;
    loop {
        s.clear();
        let r = br.read_line(&mut s);
        if r.is_err() || r.unwrap() == 0 {  // premature death
            return Err(ParseError::Unfinished);
        } else if s.starts_with("---") {
            s.clear();
            if let Err(_) = br.read_to_string(&mut s) {
                return Err(ParseError::InvalidBody);
            }
            //TODO: translate
            context.add("__content__", &s);
            return Ok(context);
        }

        let line = s.trim();
        if line.len() == 0 || line.starts_with('#') || line.starts_with("//") {
            //blank line or comment
            continue;
        }
        // key and value separated by a colon
        // colons not allowed in the key
        // (no keys in the colon either D: )
        let mid: usize = line.find(':').ok_or(ParseError::InvalidLine(count))?;
        let (first, second) = line.split_at(mid);
        let (key, value) = (first.trim(), second[1..].trim());
        context.add(key, &value);

        count += 1;
    }
}

#[derive(Debug)]
enum ParseError {
    Unfinished,
    InvalidLine(usize),
    InvalidBody,
}
