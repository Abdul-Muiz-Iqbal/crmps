use structopt::StructOpt;
mod cli;
use cli::Crmps;

type Error = Box<dyn std::error::Error>;

fn main() {
    let opt = Crmps::from_args();

    let error: Result<(), Error> = match opt {
        Crmps::New { name, template } => Ok(println!("Project {:#?} Created With {:#?} Template", name, template)),
        _ => Ok(println!("This wasn't supposed to happen"))
    };

    if let Err(e) = error {
        eprintln!("Application Errored Out: {}", e);
    }
}
