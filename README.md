# the-great-replacer

A small (despite the name) tool for replacing text in a file in all branches of a Git repository. Implemented in Rust
as a learning experience.

The tool takes the following options, which are all mandatory

* repopath, the path to the repository where the changes will be made
* filename, the name of the file that should be changed
* username, the name which will be used in the commit
* email, the email to use in the commit
* from, the text to replace 
* to, the text to insert in place of the old text
* message, the commit message to use

A commit with the changes will be made on each branch in the repository. The changes will not be pushed anywhere.
   