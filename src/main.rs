use std::io::{ stdin, stdout, Write };
use std::fs::{File, self};
use std::process;

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
    // println!("finished");
    println!("Thank you for using.");
    println!("Have a nice day!!!");
    // 終了処理を行う
    process::exit(0);
}

fn delete_file() {
    // println!("delete");
    let mut buf = String::new();
    print!("Input deletive file name → ");
    stdout().flush().unwrap();
    stdin().read_line(&mut buf).unwrap();

    let file_name = String::from("./words/") 
                            + buf.trim() 
                            + &String::from(".txt");
    // ファイルがなかったらpanic発生 後々エラー処理を作成する
    std::fs::remove_file(file_name).unwrap();
}

fn add_contents() {
    println!("add")
}

fn create_file() {
    // println!("create");
    let mut buf = String::new();
    print!("Input new file name → ");
    stdout().flush().unwrap();
    stdin().read_line(&mut buf).unwrap();

    // ↓ wordsディレクトリがない場合は作成するコードを書く

    let file_name = String::from("./words/") 
                            + buf.trim() 
                            + &String::from(".txt");
    //println!("{}", file_name);
    // もしもファイルがあったらpanicを起こす 後々エラー処理を作成する
    if fs::metadata(&file_name).is_ok() {
        panic!();
    }
    
    File::create(file_name).unwrap();
    println!("success create file");
}