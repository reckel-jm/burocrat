use burocrat::add_calculation;
use clap::{Parser,Subcommand};

/// Simple program to greet a person
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a new process from a BPMN modell
    Start { name : Option<String> },
    /// Get the status of the current process instances
    Status {},
    /// Submit information to a current process instance
    Submit { instance_name: Option<String>, submit_info: Option<String>, },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Start { name: bpmn_process_name } => {
            println!{"Start a new instance under the name {:?}", bpmn_process_name}
        },
        Commands::Status {  } => {
            println!("The status of ...");
        },
        Commands::Submit { instance_name, submit_info } => {
            println!("Submit {:?} with the info: {:?}", instance_name, submit_info);
        }
    }
}
