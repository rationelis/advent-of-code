use utils::read_input_file;

enum FileOrDirectory {
    File(Box<File>),
    Directory(Box<Directory>),
}

struct File {
    name: String,
    size: i32,
}

struct Directory {
    name: String,
    parent: Option<Box<Directory>>,
    children: Vec<Box<File>>,
    files: Vec<Box<File>>,
}

impl Directory {
    fn add_file(&mut self, file: File) {
        self.files.push(Box::new(file));
    }

    fn add_directory(&mut self, dir: Directory) {
        self.children.push(Box::new(dir));
    }
}

#[derive(Debug)]
enum Command {
    NoOp,
    Cd,
    Ls,
}

type Result<T> = std::result::Result<T, String>;

fn handle_ls_line(str: &str) -> Result<FileOrDirectory> {
    let mut parts = str.split_whitespace();

    let first = parts.next().unwrap();

    if first == "dir" {
        let name = parts.next().unwrap();
        let dir = Directory {
            name: name.to_string(),
            parent: None,
            children: Vec::new(),
            files: Vec::new(),
        };

        return Ok(FileOrDirectory::Directory(Box::new(dir)));
    }

    let size = first.parse::<i32>().unwrap();
    let name = parts.next().unwrap();
    let file = File {
        name: name.to_string(),
        size,
    };
    
    Ok(FileOrDirectory::File(Box::new(file)))
}

fn handle_new_command(str: &str) -> Result<Command> {
    let mut parts = str.split_whitespace();

    let command = parts.next().unwrap();

    match command {
        "cd" => Ok(Command::Cd),
        "ls" => Ok(Command::Ls),
        _ => Err(format!("Unknown command: {}", command)),
    }
}

fn main() {
    let lines = read_input_file("input.txt").unwrap();
    
    let mut root = Directory {
        name: "root".to_string(),
        parent: None,
        children: Vec::new(),
        files: Vec::new(),
    };

    let root_directory = Box::new(root);
    let mut current_command = Command::NoOp;
    let mut current_directory = root;

    for line in lines {
        if line.starts_with("$") {
            current_command = handle_new_command(&line).unwrap();
            continue;
        }

        match current_command {
            Command::Ls => {
                let file_or_dir = handle_ls_line(&line).unwrap();
                match file_or_dir {
                    FileOrDirectory::File(file) => {
                        root.add_file(*file);
                    },
                    FileOrDirectory::Directory(dir) => {
                        root.add_directory(*dir);
                    },
                }
            },
            Command::Cd => {
                // Find the directory in the current directory
                // If it exists, set the current directory to that directory
                // If it doesn't exist, print an error message
                // If the directory is "..", set the current directory to the parent directory
                // If the directory is /, set the current directory to the root directory
            },
            _ => {
                println!("Unknown command: {:?}", current_command);
            },
        }
    }
}

