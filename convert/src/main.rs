#[macro_use]
extern crate clap;
extern crate tera;

#[macro_use]
extern crate serde_derive;


use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::ffi::OsStr;
use tera::Tera;

//see Cobalt.rs for post layout (slightly added onto markdown)
// https://github.com/cobalt-org/cobalt.rs#posts

// args:
//  folder w/ blog posts (todo: recursive?)
//  template.html.tera: template for blog posts
//  index.html.tera: index template linking to blog posts

// will look for *.html.tera
const DEFAULT_TEMPL: &'static str = "template.html.tera";
const DEFAULT_INDEX: &'static str = "index.html.tera";
const POST_SUFFIX: &'static str = "post";

#[derive(Debug, Default, Serialize)]
struct PageData {
    title: Option<String>,
    description: Option<String>,
    date: Option<String>,
    url: Option<String>,
}

#[derive(Debug)]
enum ParseError {
    Unfinished,
    InvalidLine(usize),
    InvalidBody,
}

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
             .help("Location of files (`index`/`template.html.tera` and posts)")
             )
        .arg(clap::Arg::with_name("output")
             .takes_value(true)
             .short("o")
             .long("output")
             .help("Output folder for rendered html posts")
             )
        .arg(clap::Arg::with_name("input")
             .takes_value(true)
             .short("i")
             .long("input")
             .help("All input files to use")
             )
        .get_matches();

    //dir: path where all content (and maybe the template) is located
    let dir = Path::new(matches.value_of("dir").unwrap());
    assert!(dir.is_dir(), "<directory> must be a directory and not a file");

    let template_path = dir.join(DEFAULT_TEMPL);
    let index_path = dir.join(DEFAULT_INDEX);
    assert!(template_path.is_file());
    assert!(index_path.is_file());

    let input_dir = match matches.value_of("input") {
        None => dir,
        Some(p) => Path::new(p),
    };

    //all the actual content we'll be using
    // follow directories recursively?
    // ignore files we can't get a handle to (complain explicitly?)
    let post_type: Option<&OsStr> = Some(OsStr::new(POST_SUFFIX));
    let files: Vec<PathBuf> = fs::read_dir(input_dir)
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
    let mut page_data: Vec<PageData> = vec![];

    //only iterate through `.post` files
    for path in files {
        println!("Opening file: `{}`", path.display());
        let page_fold = match matches.value_of("output") {
            None => dir,
            Some(p) => Path::new(p)
        }.to_path_buf();
        let page_name = path.file_stem().unwrap();
        let page_path = page_fold.join(page_name).with_extension("html");
        let page = render_page(&templates, &mut page_data, &path);
        println!("Saving output to `{}`", page_path.display());
        save_page(page, &page_path).unwrap();
    }

    println!("\nPage Data: {:?}\n", page_data);
    let mut context = tera::Context::new();
    context.add("posts", &page_data);

    let index_res = templates.render(DEFAULT_INDEX, context).unwrap();
    let mut index_out = index_path.clone();
    index_out.set_extension("");
    println!("Writing result to `{}`", index_out.display());
    save_page(index_res, &index_out).unwrap();
}

fn rel_str_from_pathbuf<'a>(pb: &'a Path) -> &'a str { //bonus lifetime
    if pb.starts_with("./") {
        pb.file_name().unwrap()
    } else {
        pb.as_os_str()
    }.to_str().unwrap()
}

fn save_page(page: String, dest: &Path) -> Result<(), io::Error> {
    //writes page to destination
    //panics if weird os issue
    let mut f = fs::File::create(dest)?;
    f.write_all(page.as_bytes())
}

fn render_page(templates: &Tera, page_data: &mut Vec<PageData>, path: &PathBuf) 
        -> String {
    //can throw errors if template/context pair invalid: non-recoverable
    let f = fs::File::open(path).expect("Unable to open file");
    let mut r = BufReader::new(f);
    let context = parse_content(&mut r, page_data).expect("Failed to parse file config");
    templates.render(DEFAULT_TEMPL, context)
        .unwrap_or_else(|e| 
            panic!("Failed to apply context {:?}\n\tto template {:?}:\n\t{:?}", 
                           path, DEFAULT_TEMPL, e))
}

fn parse_content(br: &mut BufReader<fs::File>, page_data: &mut Vec<PageData>) 
        -> Result<tera::Context,ParseError> {
    let mut context = tera::Context::new();
    let mut s = String::new();
    let mut count = 0;
    let mut page_datum = PageData::default();
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
            //Success
            page_data.push(page_datum);
            //let html = markdown::to_html(&s);
            //context.add("__content__", &html);
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
        let (key, value) = (first.trim().to_lowercase(), second[1..].trim());
        context.add(&key, &value);
        match key.as_str() {
            "url"         => page_datum.url  = Some(String::from(value)),
            "date"        => page_datum.date = Some(String::from(value)),
            "title"       => page_datum.title = Some(String::from(value)),
            "description" => page_datum.description = Some(String::from(value)),
            _ => {}
        };
        count += 1;
    }
}
