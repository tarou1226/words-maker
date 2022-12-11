use std::io::{ stdin, stdout, Write };
// use std::fs::File;

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
    println!("help");
    let helper = "
Usage:
    create      make file
    add         add contents
    delete      delete file
    finished    finished this app
    ";

    println!("{}", helper)
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
    //println!("create");
    let mut buf = String::new();
    print!("Input new file name → ");
    stdout().flush().unwrap();
    stdin().read_line(&mut buf).unwrap();

    let file_name = String::from("./words/") 
                            + buf.trim() 
                            + &String::from(".txt");
    println!("{}", file_name);
}