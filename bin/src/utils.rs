use fjs_util_macros::generate_file_string;

pub fn get_routes_json_bytes() -> Vec<u8> {
    let text = generate_file_string!("default/routes.json");
    text.to_ascii_lowercase()
}

pub fn get_root_html_bytes() -> Vec<u8> {
    let text = generate_file_string!("default/index.html");
    text.to_ascii_lowercase()
}

pub fn get_root_css_bytes() -> Vec<u8> {
    let text = generate_file_string!("default/index.css");
    text.to_ascii_lowercase()
}

pub fn get_root_js_bytes() -> Vec<u8> {
    let text = generate_file_string!("default/index.js");
    text.to_ascii_lowercase()
}

pub fn get_fs_server_start_bytes() -> Vec<u8> {
let a = String::from("\
        package main

import (
    \"net/http\"
)

func serve_page(path string, folder string) {
    http.Handle(path, http.FileServer(http.Dir(folder)))
}

func main() {
    ");

    a.into_bytes()
}

pub fn get_fs_page_route_byte(routes: Vec<(String, String)>) -> Vec<u8> {
    let mut text: String = String::new(); 
    
    for (r, p) in routes.iter() {
        let a = format!("\
            serve_page(\"{}\", \"./{}/\")
    ", r, p);
        text.push_str(a.as_str());
    }

    text.into_bytes()
}

pub fn get_fs_server_end_bytes(port: &str) -> Vec<u8> {
    let text = format!("\
    http.ListenAndServe(\":{}\", nil)
}}
", port);
    text.into_bytes()
}
