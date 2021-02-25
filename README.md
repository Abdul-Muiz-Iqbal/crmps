# crmps
`crmps` is a command-line template manager for projects. It creates (from saved "templates") entire project directories with all the files and directories already created and indexed for you.

It was made in Rust, and has been made fast and versatile, so that you can avoid repeating the same commands (or worse, using the GUI) to get started with working on your projects.

## Example
Say you index your Python projects as follows:
```
project_name/  
|__ README.md
|__ src/  
    |__ main.py
```
Instead of having to type out
```
mkdir project_name
```
then
```
cd project_name
```
then
```
mkdir src
```
... 😴 Sorry I dozed off. Instead of doing that, `crmps` allows you to create all of this using:
```
crmps init project_name -template_name
```
All that you have to do is create the entire project directory once, and add it to your templates using:
```
crmps add "path/to/project/directory" -m
```
Now isn't that efficient!

## Getting Started
### Requirements
`crmps` requires the following:
- [Cargo](https://crates.io/crates/cargo) >= 1.49  
- [Rust](https://www.rust-lang.org) = 2018 Edition 
### Installation
Either `clone` the directory using `git clone https://www.github.com/Abdul-Muiz-Iqbal/crmps.git` or download it.

Now, build `crmps`. In the directory run: `cargo build --release`.

## Features
Of course, `crmps` has a ton of other features. The main ones are listed below:
- Just
- The
- Main
- Features
- Please
- Like:
- `command`: _short_ description.

## Credits
Project by [@Abdul-Muiz-Iqbal](https://github.com/Abdul-Muiz-Iqbal).

<!-- Being laconic is a gift. -->
