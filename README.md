# `crmps` A template creator
crmps stands for Commandline Rust Management Project System  
That definition has no meaning, I just wanted to name it crmps

### What is `crmps`?
- crmps is a template manager, now what is a template manager?
- an example is due here:
- say you have a file structure that you use for ALL of your Javascript Projects, such as:
- INSERT IMAGE HERE
- Now you would have to create these files again and again whenever you start a new project.
- this can get increasingly time consuming with more complex directory structures
- `crmps` solves this issue with a simple command.
- just create a template directory that you always use and place it in crmps's template directory
- now from anywhere, you can do:
`$ crmps init ProjectName -TemplateName`  
- This will create a folder ProjectName that will contain content according to the TemplateName file specified.

### Is that all?
- yup, that's crmps main feature for now. it includes some extra stuff that's more or less experimental until version 1.1
