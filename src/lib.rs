use zip_extensions::*;
use std::{
    path,
    fs,
    error::Error,
    env::Args
};

const DEFAULT_PATH: &str = r"C:\Users\Muiz\Desktop\Cramps\Templates";

/// The configuration or structure of the command line args
pub struct Config {
    pub command: String,
    pub arg: String,
    pub flag: String
}
impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
         .field(&self.command)
         .field(&self.arg)
         .field(&self.flag)
         .finish()
    }
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        args.next(); // Get Rid Of The Current Working Dir
        // Make Sure Commands, Arguments And Flags Are Given
        let command = match args.next() {
            Some(val) => val,
            None => return Err("Command Not Given")
        };

        let arg = match args.next() {
            Some(val) => val,
            None => return Err("Argument Not Given")
        };

        let flag = match args.next() {
            Some(val) => val,
            None => String::new()
        };
        let config = Config {command, arg, flag};
        //println!("{:#?}", config);
        Ok(config) // Return A Config In A Result
    }
}

/// The names of all commands as a CommandNames enum
pub enum CommandNames {
    Init,
    Pack,
    Unpack,
    Tag,
    Untag,
    AddTemplate,
    RemoveTemplate,
    Search,
    Nil
}
impl CommandNames { // Map String Command To Enum Variant
    pub fn new(com: &str) -> CommandNames {
        let com = com.to_lowercase();

        if com == String::from("init") {
            CommandNames::Init
        } else if com == String::from("pack") {
            CommandNames::Pack
        } else if com == String::from("unpack") {
            CommandNames::Unpack
        } else if com == String::from("tag") {
            CommandNames::Tag
        } else if com == String::from("untag") {
            CommandNames::Untag
        } else if com == String::from("add") {
            CommandNames::AddTemplate
        } else if com == String::from("rem") {
            CommandNames::RemoveTemplate
        } else if com == String::from("search") {
            CommandNames::Search
        } else {
            CommandNames::Nil // Command Not Found
        }        
    }
}

///Type Alias
type RecoverableErr = Result<(), Box<dyn Error>>;
/// Commands available in crmps in the CommandBody header
pub struct CommandBody;
impl CommandBody {
    //crmps init Project -impl
    pub fn init(config: Config) -> RecoverableErr {
        // Get Project Name
        let name = config.arg;
        // Check If Project Exists:
        let path = path::Path::new(&name);
        if path.exists() {
            //    Yes: Return Error
            eprintln!("Project Already Exists!");
            std::process::exit(0);
        }else {
            //    No: Create Folder With Project Name
            fs::create_dir(&name)?;
            // Check For Flag:
            if config.flag != String::new() {
                //    Yes: Find If Flag In Templates Or Abs Path Given
                let path = path::Path::new(&config.flag);
                let templates_dir = path::Path::new(DEFAULT_PATH).join(path);
                if path.exists() { // ABS Path Given
                    //    Parse Dir
                    copy_dir_custom(&path, path::Path::new(&std::env::current_dir()?.join(&name)))?;
                }else if templates_dir.exists() { // Template Dir Given
                    //    Parse Dir
                    copy_dir_custom(&templates_dir, path::Path::new(&std::env::current_dir()?.join(&name)))?;
                }
                // Check If Flag Is .str:
                //    Yes: Parse .str File
            }else {
                //    No: Return Error
                eprintln!("Error! No Template Specified");
                std::process::exit(0);
            }
        }
        Ok(())
    }
    // crmps unpack Project C:/Users/Muiz/Desktop
    pub fn unzip(config: Config) -> RecoverableErr {
        // Get Proect Name
        let name = config.arg;
        // Check If Project Exists:
        let path = path::PathBuf::new();
        let path = path.join(path::Path::new(&name));
        if path.exists() {
            //    Yes: Check If unPack To Cur Working Dir
            if config.flag != String::from("-c") {
                let target = path::PathBuf::new();
                let target = target.join(&config.flag);
                if target.exists() {
                    //        No: unPack To Flag Path
                    zip_extract(&path, &target)?;
                }else {
                    eprintln!("Error! Project Not Found");
                    std::process::exit(0);                    
                }
            }else {
                //        Yes: unPack To Cur Working Dir
                zip_extract(&path, &std::env::current_dir()?)?;
            }

        }else {
            //    No: Return Error
            eprintln!("Error! Project Not Found");
            std::process::exit(0);
        }
        Ok(())
    }
    // crmps pack Project -c
    pub fn zip(config: Config) -> RecoverableErr {

        // Get Project Name
        let name = config.arg;
        // Check If Project Exists:
        let path = path::Path::new(&name);
        if path.exists() {
            //    Yes: Check If pack To Cur Working Dir
            if config.flag == String::from("-c") {
                //        Yes: pack To Cur Working Dir
                println!("{:#?}", path);
                let archive_dir = format!("{}.rar", path.file_name().unwrap().to_str().unwrap());
                println!("{}", archive_dir);
                zip_create_from_directory(&std::env::current_dir()?.join(archive_dir), &path::PathBuf::new().join(&path))?;
            }else {
                //        No: pack To Flag Path
                let archive_dir = path::PathBuf::new().join(&config.flag);
                let src = path::PathBuf::new().join(&path);
                println!("{:#?}[]{:#?}", archive_dir, src);
                zip_create_from_directory(&archive_dir, &src)?;
            }
        }else {
            //    No: Return Err
            eprintln!("Error! Can't Find Project");
            std::process::exit(0);
        }
        Ok(())
    }
    // crmps tag ./JSProject Python ///TODO: Add Listing Multiple Tags
    pub fn tag(config: Config) -> RecoverableErr {
        // Get Project Name
        let name = config.arg;
        let path = path::Path::new(&name);
        // Check If Exists
        if path.exists() {
            //     Yes: Get Tag Name
            let tag = config.flag;
            let tag_file = path.join(".tags");
            //     Check If TagFile Exists:
            if tag_file.exists() {
                //        Yes: Check If Tag Already Exists
                let contents = fs::read_to_string(&tag_file)?;
                if contents.contains(&tag) {
                    //            Yes: Error
                    eprintln!("Project {} Already Tagged With {}", name, tag);
                    std::process::exit(0);
                } else {
                    //            No: Add Tag
                    fs::write(tag_file, format!("{}{}\n", contents, tag))?;
                }
            }else {
                //        No: Create New TagFile + AddTag
                fs::write(tag_file, format!("{}\n", tag))?;
            }
        } else {
            //    No: Error        
            eprintln!("Project {} Not Found", name);
            std::process::exit(0);
        }
        Ok(())
    }
    // crmps untag ./JSProject Python ///TODO: Add Listing Multiple Tags
    pub fn untag(config: Config) -> RecoverableErr {
        // Get Project Name
        let name = config.arg;
        // Check If Exists
        let path = path::Path::new(&name);
        if path.exists() {
            //     Yes: Get Tag Name
            let tag = config.flag;
            let tag_file = path.join(".tags");
            if tag_file.exists() {
                //     Check If Tag Exists:
                let contents = fs::read_to_string(&tag_file)?;
                if contents.contains(&tag) {
                    //        Yes: Remove Tag
                    fs::write(&tag_file, format!("{}\n", contents.replace(&tag, "").trim()))?;
                } else {
                    //        No: Return Error
                    eprintln!("Tag Already Removed/Not Existed");
                    std::process::exit(0);
                }
            } else {
                eprintln!("Error TagFile Not Found");
                std::process::exit(0);
            }
        }else {
            //    No: Error
            eprintln!("Project Not Found");
            std::process::exit(0);
        }
        Ok(())
    }
    // crmps add template_to_add -m
    pub fn add(config: Config) -> RecoverableErr {
        // Get Template File Path
        let name = config.arg;
        // Check If File Exists:
        let path = path::Path::new(&name);
        if  path.exists() { 
            // Yes: Copy File
            copy_dir(path, path::Path::new(DEFAULT_PATH))?;
            //    Yes: Check If Move:
            if config.flag == String::from("-m") {
                //        Yes: Move File To Templates
                if path.is_dir() {
                    fs::remove_dir_all(path)?;
                }else {
                    fs::remove_file(path)?;
                }
            }

        }else {
            //    No: Return Error
            eprintln!("Error! Path {:#?} Not Found", path);
            std::process::exit(0);
        }
        Ok(())
    }
    // crmps rem template_to_remove
    pub fn rem(config: Config) -> RecoverableErr {
        
        // Get Template Name
        let name = &config.arg;
        // Check If Template Exists:
        let template_path = path::Path::new(name);
        let local_default_path = path::Path::new(&DEFAULT_PATH);
        // Is In Default Dir:
        let template_path = if local_default_path.join(template_path).exists() {
            local_default_path.join(template_path)
        } else { // Otherwise, An Absolute Existing Path Is Specified
            path::PathBuf::new().join(template_path)
        };
        //    Yes: Delete
        if let Some(_) = template_path.extension() {
          fs::remove_file(template_path)?;  
        }else {
            fs::remove_dir_all(template_path)?;
        }
        Ok(())
    }

    pub fn search(_config: Config) -> RecoverableErr {
        Ok(())
    }
}

fn copy_dir(path: &path::Path, dest: &path::Path) -> RecoverableErr {
    // Get Dest Dir Name
    let dest_name: String = match path.file_name() {
        Some(v) => String::from(v.to_str().unwrap()),
        None => {
            eprintln!("Invalid Path: {:#?}", path);
            std::process::exit(0);
        }
    };
    // Creaate Dest Dir
    let dest = dest.join(dest_name);
    //let local_default_path = path::Path::new(&DEFAULT_PATH);
    fs::create_dir(&dest)?;
    // Loop Through Each Level Of Src Dir
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name().to_str().unwrap());
        //println!("{:#?}, {:#?}", dest_path, src_path);
        // If Is Dir, Create Dir In Dest With Correct Path
        if src_path.is_dir() {
            fs::create_dir(&dest_path)?;
            copy_dir(path::Path::new(src_path.to_str().unwrap()), path::Path::new(&dest_path.to_str().unwrap()))?;
        }else { // If Is File, Copy / Copy|Delete To Dest Path With Correct Path
            fs::copy(src_path, dest_path)?;
        }

    }
    Ok(())
}
fn copy_dir_custom(path: &path::Path, dest: &path::Path) -> RecoverableErr {
    // Get Dest Dir Name
    //let local_default_path = path::Path::new(&DEFAULT_PATH);
    // Loop Through Each Level Of Src Dir
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name().to_str().unwrap());
        //println!("{:#?}, {:#?}", dest_path, src_path);
        // If Is Dir, Create Dir In Dest With Correct Path
        if src_path.is_dir() {
            fs::create_dir(&dest_path)?;
            copy_dir(path::Path::new(src_path.to_str().unwrap()), path::Path::new(&dest_path.to_str().unwrap()))?;
        }else { // If Is File, Copy / Copy|Delete To Dest Path With Correct Path
            fs::copy(src_path, dest_path)?;
        }

    }
    Ok(())
}