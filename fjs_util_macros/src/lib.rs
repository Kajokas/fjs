use std::fs;

use proc_macro::{Literal, TokenStream, TokenTree};

/// Takes file path to file pressent in bin/ (the directory name of fjs cli bin) and returns its
/// bytes.
///
/// Because Rust macros only trigger with code change just changing the file will not update the
/// bytes returned by this macro.
#[proc_macro]
pub fn generate_file_string(input: TokenStream) -> TokenStream {
    let mut file_bytes: Literal = Literal::byte_string(b"");
    for i in input.into_iter() {
        match i {
            TokenTree::Literal(x) => {
                let mut file_path = x.to_string();
                file_path.remove(0);
                file_path.pop();

                let file_path = format!("./bin/{}", file_path);
                let file = fs::read(file_path.clone()).expect(format!("Unable to open file {}", file_path.as_str()).as_str());

                file_bytes = Literal::byte_string(&file);
            }
            _ => panic!("generate_file_string only accepts file paths")
        };
    }

    TokenStream::from(TokenTree::from(file_bytes))
}
