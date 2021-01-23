## `crmps` - A Template Manager
---
### What is `crmps`?
   `crmps` is a template manager made in [Rust](https://www.rust-lang.org). Its fast, predictable and versatile. `crmps` allows you to create folders that you can use as templates for other projects. In this way, there is little need of repeating the same CLI commands just to create a basic project. Say you organize your python scripts as follows:
   ```
   DirName/  
   |__ src/  
       |__ main.py  
       |__ lib.py  
   ```
   Translating this into CLI commands, this structure can be created as follows:
   ```
   $ mkdir DirName/
   $ mkdir DirName/src
   $ touch DirName/src/main.py
   $ touch DirName/src/lib.py
   ```
   This can get big really fast, especially if you like a lot of directories. With `crmps`, all you have to do is create a template for this structure:
   ```
   py/
   |__ src/
       |__ main.py
       |__ lib.py
   ```
   Next you have to move this template to `crmps` database:
   ```
   $ crmps add "PathToTemplate" -m
   ```
   Finally to create a new project with this template:
   ```
   $ crmps init ProjectName -templateName
   ```
   Therefore, using our previous example:
   ```crmps new DirName -py```
   `crmps` also has numerous other quality of life features, described below in the Usage section.
### Getting Started
   Make sure you have cargo installed. Run the commands:  
    ```$ git clone https://www.github.com/Abdul-Muiz-Iqbal/crmps.git```  
    ```$ cd crmps```  
    ```$ cargo build --release```  
    Or alternatively, download the [release binary](https://www.github.com/Abdul-Muiz-Iqbal) (NOT_CREATED_YET)
### Requirements
   - [Cargo](https://crates.io/crates/cargo) >= 1.49  
   - [Rust](https://www.rust-lang.org) = 2018 Edition  
### Usage
   - Creating a new project:  
        ```$ crmps new ProjectName -TemplateName```  
   - Adding a new template:  
        ```$ crmps add PathToTemplate```  
   - Removing a template:  
        ```$ crmps rem "PathToTemplate (Can be an asterisk (*) to remove all templates )"```  
   - Tagging (A way to easily find projects) a project:  
        ```$ crmps tag ProjectName --with "#Python #Django #Oops"```  
   - Untagging tags from a project (Use * to remove all tags):  
        ```$ crmps untag ProjectName --tags "#Oops"```  
   - Locating a project by name or by tags:  
        ```$ crmps locate "DirName" --in "DirNameOr -d"```  
        ```$ crmps locate "#Python #Django"```  
   - Listing the contents of a directory till the specfied recursion limit:  
        ```$ crmps ls --till 5```  
   - Set a directory for crmps to default to when using the -d flag:  
        ```$ crmps default "D:/Code"```  
   - Register a command to execute when you give crmps a file with a specific extension (Use {{fileName}} to replace with filename):  
        ```$ crmps register --extension ".js" --command "node {{fileName}}"```  
   - Save a file as a new subcommand to crmps:  
        ```$ crmps register --subcommand "runBot" --file "Path/To/File/Bot.js"```  
   - Invoke a custom command:  
        ```$ crmps runBot```  
Created By [@Abdul-Muiz-Iqbal](https://www.github.com/Abdul-Muiz-Iqbal)
