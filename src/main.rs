pub mod database;
pub mod cli;
use clap::Parser;
use cli::{Cli,Commands};
use database::Database;

fn main() {
    let args = Cli::parse();
    let mut db = Database::open(".rododb");
        match args.command{ 
        Commands::Delete { id } => {
            if id.is_none(){
                println!("You need to specify the id of the todo item.");
                return;
            }
            println!("Removing todo item with id:{}",id.clone().unwrap());
            db.remove_record(id.unwrap().parse::<i32>().unwrap());
        }
        Commands::Add{ content }=>{
            if let Some(content) = content{
                println!("Adding todo item: {}",content);
                let id = db.read_record().last().map(|r|r.id + 1).unwrap_or(1);
                db.add_record(&database::Record { id: id, content,checked:false});
            }
            else{
                println!("You need to specify the content of the todo item")
            }    
        }
        Commands::Check{ id }=>{
            if id.is_none(){
                println!("You need to specify the id of the todo item.");
                return;
            }
            println!("Checking todo item with id:{}",id.clone().unwrap());
            db.check_record(id.unwrap().parse::<i32>().unwrap());
        }
        Commands::Info=>{
            println!("Rodo is a simple todo list manager");
        }
        Commands::List=> {
            let records = db.read_record();
            if records.is_empty(){
                println!("nothing yet");
                return;
            }
            for record in records{
                println!("id:{},content:{},checked:{}",&record.id,&record.content,&record.checked);
            }
            println!("List!");
        }
        Commands::Remove=>{
            db.remove_all_record();
        }
    }

}
