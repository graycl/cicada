use std::path::Path;

use sqlite;
use sqlite::State;

use history;
use tools;

pub fn run(args: Vec<String>) -> i32 {

    let hfile = history::get_history_file();
    let path = Path::new(hfile.as_str());
    if !path.exists() {
        tools::println_stderr("no history file.");
        return 1;
    }

    if let Ok(conn) = sqlite::open(hfile.clone()) {
        if args.len() == 1 {
            return list_current_history(conn);
        }
        else if args.len() == 2 {
            search_history(conn, args[1].clone());
        } else {
            tools::println_stderr("history: only take one arg");
        }
    } else {
        tools::println_stderr("history: history file open error.");
        return 1;
    }
    return 0;
}

fn list_current_history(conn: sqlite::Connection) -> i32 {
    let q = "SELECT inp FROM xonsh_history ORDER BY tsb desc limit 10;";
    match conn.prepare(q) {
        Ok(mut statement) => {
            let mut vec = Vec::new();
            while let State::Row = statement.next().unwrap() {
                vec.push(statement.read::<String>(0).unwrap());
            }
            for (i, elem) in vec.iter().rev().enumerate() {
                println!("{}: {}", i, elem);
            }
        }
        Err(e) => {
            tools::println_stderr(format!("history: prepare error - {:?}", e).as_str());
            return 1;
        }
    }
    return 0;
}

fn search_history(conn: sqlite::Connection, q: String) {
    let q = format!("SELECT inp FROM xonsh_history
                     WHERE inp like '%{}%'
                     ORDER BY tsb desc limit 20;", q);
    let mut statement = conn.prepare(q).unwrap();
    let mut vec = Vec::new();
    while let State::Row = statement.next().unwrap() {
        vec.push(statement.read::<String>(0).unwrap());
    }
    for (i, elem) in vec.iter().rev().enumerate() {
        println!("{}: {}", i, elem);
    }
}