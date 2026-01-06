use fjs_util_macros::generate_file_string;

pub fn get_fs_server_file_bytes() -> Vec<u8> {
    let text = generate_file_string!("default/main.go");
    text.to_vec()
}

pub fn get_fs_server_start_bytes(fs_server_bytes: Vec<u8>) -> Vec<u8> {
    let a = String::from_utf8(fs_server_bytes).expect("Counldn't convert fs to string");
    let b:Vec<&str> = a.split("@@GENERATE_CODE").collect();
    let b = b.first().expect("Missing\"@@GENERATE_CODE\"");

    String::from(*b).into_bytes()
}

pub fn get_fs_page_route_byte(routes: Vec<(String, String)>) -> Vec<u8> {
    let mut text: String = String::new(); 
    
    for (r, p) in routes.iter() {
        let a = format!("serve_page(\"{}\", \"./{}/\")\n    ", r, p);
        text.push_str(a.as_str());
    }

    text.into_bytes()
}

pub fn get_fs_server_end_bytes(port: &str) -> Vec<u8> {
    let text = format!("http.ListenAndServe(\":{}\", nil)\n}}", port);
    text.into_bytes()
}
