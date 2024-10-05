use clap::{Parser,Subcommand};
//Parser:parse command-line arguements into Self.
//Subcommand:Parse a sub-command into a user-defined enum.
//Struct Command:::When deriving a Parser, you can use CommandFactory::command to access the Command.

#[derive(Parser)]
#[clap(version,about)]
#[clap(propagate_version = true)]
pub struct Cli{
    #[clap(subcommand)]
    pub command: Commands,
}
#[derive(Debug,Subcommand)]
pub enum Commands{
    #[clap(about = "Show rodo info.")]
    Info,

    #[clap(about = "Add a todo item.")]
    Add{
        #[clap(help = "The item content to add.")]
        content: Option<String>,
    },

    #[clap(about = "Check a todo item.")]
    Check{
        #[clap(help = "the item content finished.")]
        id :Option<String>,

    },

    #[clap(about = "Delete a todo item.")]
    Delete{
        #[clap(help = "the item content to delete.")]
        id:Option<String>,
    },

    #[clap(about = "List all the todo items.")]
    #[clap(visible_aliases = & ["ls","la","ll"])]
    List,

    #[clap(about = "Remove all items.")]
    #[clap(visible_aliases = & ["rm"])]
    Remove,
}
