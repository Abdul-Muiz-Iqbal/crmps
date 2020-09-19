// crmps - Template Creator
// Licensed Under Apache 2.0
// Written By: (Abdul-Muiz-Iqbal)[https://www.github.com/Abdul-Muiz-Iqbal]

// API
// TODO:
// - Implement Most Methods
// - Refactor Already Implemented Methods

mod lib;
use std::process;
use lib::*;


fn main() {
    let config = match Config::new(std::env::args()) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(0);
        }
    };
    
    let error = match CommandNames::new(&config.command) {
        CommandNames::Init => CommandBody::init(config),
        CommandNames::Pack => CommandBody::zip(config),
        CommandNames::Unpack => CommandBody::unzip(config),
        CommandNames::Tag => CommandBody::tag(config),
        CommandNames::Untag => CommandBody::untag(config),
        CommandNames::AddTemplate => CommandBody::add(config),
        CommandNames::RemoveTemplate => CommandBody::rem(config),
        CommandNames::Search => CommandBody::search(config),
        CommandNames::Nil => {
            eprintln!("Command Not Found");
            std::process::exit(0)
        }
    };

    if let Err(e) = error {
        eprintln!("Application Error: {:#?}", e);
        std::process::exit(0);
    }
}
