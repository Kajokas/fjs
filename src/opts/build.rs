use std::{fs::{self, File}, io::Write, path::Path};

use crate::{opts::tmp::copy_dir_all, utils};

fn build_prj(path: String, build_extra_route: Option<String>) {
    let filename = format!("{}/main.go", path.as_str());
    let mut fs_server_file = File::create(filename).expect("Failed to make file server file");

    let mut file_bytes: Vec<u8> = Vec::new(); 

    let mut routes: Vec<(String, String)> = Vec::new();

    let r = String::from("/");
    let mut p = build_extra_route.unwrap_or(String::from(""));
    p.push_str("root");
    routes.push((r, p));

    file_bytes.extend_from_slice(&utils::get_fs_server_start_bytes());
    file_bytes.extend_from_slice(&utils::get_fs_page_route_byte(routes));
    file_bytes.extend_from_slice(&utils::get_fs_server_end_bytes("3000"));

    fs_server_file.write(&file_bytes).expect("Failed to write data to file");
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

