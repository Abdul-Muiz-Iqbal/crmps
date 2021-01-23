use structopt::StructOpt;
mod cli;
use cli::Crmps;

type Error = Box<dyn std::error::Error>;

fn main() {
    let opt = Crmps::from_args();

    let error: Result<(), Error> = match opt {
        Crmps::New { name, template } => Ok(println!("Project {:#?} Created With {:#?} Template", name, template)),
        _ => Ok(println!("Who with the what now?"))
    };

    if let Err(e) = error {
        eprintln!("Application Errored Out: {}", e);
    }
}
