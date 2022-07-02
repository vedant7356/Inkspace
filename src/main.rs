use std::fs;
use std::io::{LineWriter, Write};

mod html_gen;
mod md_parser;

fn write_to_template(content: Vec<String>) {
    let file_handle = fs::File::create("/home/bourbon/dev/Inkspace/templates/gen_index.html.tera")
        .expect("File not found!");

    let mut file_handle = LineWriter::new(file_handle);

    for tag in content {
        writeln!(&mut file_handle, "{}", tag)
            .expect("Failed to write to the file!");
    }

    file_handle.flush().unwrap();
}

fn main() {
    //let content: Vec<String> = html_gen::call_generator();
    //write_to_template(content);
    html_gen::call_generator();
} 
