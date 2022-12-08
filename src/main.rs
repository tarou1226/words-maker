use std::io::{ stdin, stdout, Write };

fn main() {
    loop {
        let mut buf = String::new();
        print!("Input command → ");
        stdout().flush().unwrap();
        stdin().read_line(&mut buf).unwrap();
        match buf.trim() {
            "create" => create_file(),
            "add" => add_contents(),
            "delete" => delete_file(),
            "finished" => finished(),
            _ => help(),
        }
    }
}

fn help() {
    println!("help")
}

fn finished() {
    println!("finished")
}

fn delete_file() {
    println!("delete")
}

fn add_contents() {
    println!("add")
}

fn create_file() {
    println!("create")
}
