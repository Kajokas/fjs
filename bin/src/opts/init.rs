use std::{fs::{self, File}, io::Write};
use super::utils;

fn create_prj(path: String) {
    let filename = format!("{}/routes.json", path.as_str());
    let mut routes_file = File::create(filename).expect("Failed to create routes file");

    let mut file_bytes: Vec<u8> = Vec::new(); 

    file_bytes.extend_from_slice(&utils::get_routes_json_bytes());

    routes_file.write(&file_bytes).expect("Failed to write data to routes.json");

    let path = format!("{}/pages", path);
    fs::create_dir(&path).expect("Failed to create a dir");
    create_root(&path);
}

fn create_root(path: &str) {
    let path = format!("{}/root", path);
    fs::create_dir(&path).expect("Failed to create a dir");

    let html_filename = format!("{}/index.html", path.as_str());
    let css_filename = format!("{}/index.css", path.as_str());
    let js_filename = format!("{}/index.js", path.as_str());

    let mut root_html_file = File::create(html_filename).expect("Failed to make file html file");
    let mut root_css_file = File::create(css_filename).expect("Failed to make file css file");
    let mut root_js_file = File::create(js_filename).expect("Failed to make file js file");

    root_html_file.write(&utils::get_root_html_bytes()).expect("Failed to write data to file");
    root_css_file.write(&utils::get_root_css_bytes()).expect("Failed to write data to file");
    root_js_file.write(&utils::get_root_js_bytes()).expect("Failed to write data to file");
}

pub fn init(args: Vec<String>) {
    if args.len() <= 2 {
        println!("Initializing project in current dir");

        create_prj(String::from("."));
    } else {
        println!("Creating project {}", args[2]);

        let path = format!("./{}", args[2]);
        fs::create_dir(&path).expect("Failed to create a dir");
        create_prj(path);
    }
}

