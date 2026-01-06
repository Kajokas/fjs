use std::{fs::{self, File}, io::Write, path::Path};

use serde_json::Value;

use crate::{opts::tmp::copy_dir_all, utils};

fn build_prj(path: String, build_extra_route: Option<String>) {
    let filename = format!("{}/main.go", path.as_str());
    let mut fs_server_file = File::create(filename).expect("Failed to make file server file");

    let mut file_bytes: Vec<u8> = Vec::new(); 

    let routes: Vec<(String, String)> = get_routes(build_extra_route);

    file_bytes.extend_from_slice(&utils::get_fs_server_start_bytes());
    file_bytes.extend_from_slice(&utils::get_fs_page_route_byte(routes));
    file_bytes.extend_from_slice(&utils::get_fs_server_end_bytes("3000"));

    fs_server_file.write(&file_bytes).expect("Failed to write data to file");
}

fn get_routes(build_extra_route: Option<String>) -> Vec<(String, String)> {
    let mut routes: Vec<(String, String)> = Vec::new();

    let routes_json_data = fs::read("./routes.json").expect("Failed to read routes json");
    let routes_map:Value = serde_json::from_slice(&routes_json_data).expect("Failed to get routes from json");
    let routes_map = routes_map.as_object().expect("Json format invalid in routes.json");

    for (route_uri, route_dir) in routes_map.iter() {
        let r = String::from(route_uri);
        let mut p = build_extra_route.clone().unwrap_or(String::from(""));
        p.push_str(route_dir.as_str().expect("Route page path must point to a real folder"));
        routes.push((r, p));
    }

    routes
}

pub fn build(build_type: Option<String>) {
    if fs::exists("./build").expect("Failed to verify previouse build existance") {
        fs::remove_dir_all("./build").expect("Failed to delete previous build");
    }

    println!("Now building");
    fs::create_dir("build").expect("Failed to create a dir");

    let extra_path = match build_type {
        Some(x) => match x.as_str() {
            "debug" => Some(String::from("build/")),
            _ => None
        }
        None => None
    };

    build_prj(String::from("./build"), extra_path);

    let source = Path::new("./pages");
    let destination = Path::new("./build");
    copy_dir_all(source, destination).unwrap();
}

