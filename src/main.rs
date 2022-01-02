extern crate chrono;
extern crate diesel;
extern crate dotenv;
extern crate num;

mod infrastructure;
mod domain;
mod application;
mod presentation;
use std::collections::HashMap;
use std::io::BufReader;

extern crate getopts;
use getopts::Options;
use serde_json::de::StrRead;
use std::env;
use std::fs::File;
use std::fs::read_to_string;
use std::path::Path;

use infrastructure::calender;

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let file = File::open("db.json").expect("file not found");
        let reader = BufReader::new(file);
        //let deseriarized: Todo = serde_json::from_reader(reader).expect("error while deseriarizing json");
        //for (key,value) in deseriarized.map.iter() {
        //    println!("{} {}", key.unwrap(), value);
        //}
        //let db_path = Path::new("db.txt");
        //let json_str = read_to_string(db_path).expect("file not found");
        //let ToDo: data = serde_json::from_reader(&json_str).expect("error while reading json");
        
        match serde_json::from_reader(reader) {
            Ok(map) => Ok(Todo {map}),
            Err(e) if e.is_eof() => Ok(Todo { map: HashMap::new() }),
            Err(e) => panic!("An error occured: {}", e),
        }        
    }

        // read its content into a new string
        //let mut content = String::new();
        //f.read_to_string(&mut content)?;

        // allocate an empty HashMap
        //let mut map = HashMap::new();

        // loop over each lines of the file
        //for entries in content.lines() {
            // split and bind values
        //    let mut values = entries.split('\t');
        //    let key = values.next().expect("No Key");
        //    let val = values.next().expect("No Value");
            // insert them into HashMap
        //    map.insert(String::from(key), bool::from_str(val).unwrap());    
        
        // Return Ok
    //    Ok(Todo { map })
    //    }

        // another method : more "functional" approach
        //let map: HashMap<String, bool> = content
        //    .lines()
        //    .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
        //    .map(|v| (v[0], v[1]))
        //    .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
        //    .collect();
        //Ok(Todo { map })

    fn insert(&mut self, key: Option<String>) {
        self.map.insert(key.unwrap(), true);
    }

    //fn save(self) -> Result<(), std::io::Error> {
    //    let mut content = String::new();
    //    for (k, v) in self.map {
    //        let record = format!("{}\t{}\n", k, v);
    //        content.push_str(&record)
    //    }
    //    std::fs::write("db.txt", content)
    //}
    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        // open db.json
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;
        // write to file with serde
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn run_todo_app() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optopt("a", "", "set action: add, complete", "ACTION");
    opts.optopt("i", "", "set item to identify todo item", "ITEM");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("") }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let action = matches.opt_str("a");
    let item = matches.opt_str("i");
    
    if action.is_none() || item.is_none() {
        println!("Both ACTION and ITEM must be given.");
        print_usage(&program, opts);
        return;
    }

    println!("{:?}, {:?}", action, item);

    let mut todo = Todo::new().expect("Initialisation of db failed");    

    match action {
        Some(ref str_action) => match &str_action[..] {
            "add" => {
                todo.insert(item);
                match todo.save() {
                    Ok(_) => println!("todo saved"),
                    Err(why) => println!("An error occurred: {}", why),
                }
            },
            "complete" => {
                let item_org = item.clone();                
                match todo.complete(&item.unwrap()) {
                    None => println!("ITEM: {} is not present in the list", item_org.unwrap()),
                    Some(_) => match todo.save() {
                        Ok(_) => println!("todo saved"),
                        Err(why) => println!("An error occurred: {}", why),
                    },
                }
            },
            _ => {
                println!("{} is not ", str_action)
            }
        },
        _ => panic!("There was a problem to read ACTION")
    }
}

fn test_calender_function() {

}

fn main() {
    //run_todo_app();

    calender::cli_calender();
}
