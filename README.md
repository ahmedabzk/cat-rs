# cat-rs
 The cat command is used to create and view text files on the command line in Linux.
 But let's assume you have three text files: file1.txt, file2.txt, and file3.txt.
 You want to combine (or concatenate) them into one text file containing information from all three, in that order. You can do this with the cat command as well.
 I have implemented the cat command in rust.

## Installation

 First, ensure you have Rust installed on your system. If not, download it from the official Rust website.

Then, clone the project and build it using `cargo`:

```bash
git clone git@github.com:ahmedabzk/cat-rs.git
cd cat-rs
cargo build --release
```

This will create an executable in the `target/release` directory.

## Usage

To start the editor, run:

```bash
./target/release/cat-rs
```

To open the content of a specific file in the terminal, pass the file name as an argument:

```bash
./target/release/cat-rs file.txt
```

To combined contents of one or more text files  into another text file,run the below command. It will combine the contents of file1.txt and file2.txt into file3.txt.

```bash
./target/release/cat-rs file1.txt file2.txt > file3.txt
```

To add a bit of new text to an existing text file, you use the cat command to do it directly from the command line.

```bash
./target/release/cat-rs >> file3.txt
```

## Features

- [ ] Concatenation of files (file1.txt, file2.txt, etc.)
- [ ] Adding a new text from to a file from the terminal
- [ ] Printing the contents of a file to a terminal
- [ ] Concatenating contents in alphabetical order.
