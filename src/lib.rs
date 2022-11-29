extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro::TokenTree::Literal;
use std::fs;

use std::str;

#[proc_macro]
pub fn txt_to_enum(__stream: TokenStream) -> TokenStream {

    let mut stream = __stream.into_iter();

    let path = match stream.next().expect("No path given") {
        Literal(path) => path.to_string(),
        _ => return "compile_error!(\"No valid path given\")".parse().unwrap()
    };
    
    let path = &path[1..path.len()-1];
    
    let path = format!("{}/src/{}", env!("CARGO_MANIFEST_DIR"), path.clone());

    match fs::read_to_string(path.clone()) {
        Ok(data) => {
            let mut data = data.lines();
            let name = data.next();
            let joined = data.collect::<Vec<&str>>().join(",\n");

            match name {
                Some(name) => {
                    format!("enum {} {{ {} }}", name, joined).parse().unwrap()
                }
                None => {
                    "compile_error!(\"File must contain atleast one Line\")".parse().unwrap()
                }
            }
            
        }
        Err(err) => {
            format!("compile_error!(\"{}\")", err).parse().unwrap()
        }
    }
    
    
}