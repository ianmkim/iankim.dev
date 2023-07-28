use rocket::*;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;
use std::path::Path;
use std::fs::File;
use serde::Serialize;
use std::io::{
    self,
    BufRead,
};

fn read_first_line(file_path: &str) -> io::Result<Option<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let first_line = reader.lines().next();

    match first_line {
        Some(line) => Ok(Some(line.unwrap())),
        None => Ok(None),
    }
}


fn get_file_paths(dir_path: &Path) -> Vec<String> {
    let mut file_paths = Vec::new();

    for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(file_path) = path.to_str() {
                file_paths.push(file_path.to_string());
            }
        }
    }

    file_paths
}


#[derive(Serialize)]
struct Preview {
    order: usize,
    title: String,
    preview: String,
    link: String,
}


#[get("/")]
pub fn blogs_fn() -> Template {
    let mut context = HashMap::new();
    let mut stories = Vec::<Preview>::new();
    for (i, path) in get_file_paths(Path::new("blogs")).iter().enumerate(){
        let mut story = Preview {
            order: i + 1,
            title: "".to_string(),
            preview: "".to_string(),
            link: path.replace("blogs/", "blog/read/").replace(".md", ""),
        };
        match read_first_line(&path) {
            Ok(Some(first_line)) => {
                story.title = first_line.replace("# ", "");
            }
            Ok(None) => {
                println!("The file is empty.");
            }
            Err(err) => {
                eprintln!("Error reading the file: {:?}", err);
            }
        }
        stories.push(story);
    }

    context.insert("posts", stories);
    Template::render("blog_list", &context)
}

#[get("/read/<title>")]
pub fn read_fn(title:String) -> Template {
    let mut context = HashMap::new();
    let mut url:String = "".to_string();
    let escaped_title = url_escape::encode_component_to_string(title, &mut url);
    println!("{}", escaped_title);
    let load_result = fs::read_to_string(format!("blogs/{}.md", escaped_title));
    let markdown_result = match load_result {
        Ok(markdown) => markdown,
        Err(_e) => format!("## 404: Could not find post\nCouldn't find post titled {}.", url)
    };
    let html = markdown::to_html_with_options(&markdown_result, &markdown::Options::gfm()).unwrap();
    context.insert("raw_post", html);
    Template::render("blog", &context)
}
