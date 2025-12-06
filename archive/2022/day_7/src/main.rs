use utils::read_input_file;

enum FileOrDirectory {
    File(Box<File>),
    Directory(Box<Directory>),
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: i32,
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    parent: *const Directory,
    sub_directories: Vec<Box<Directory>>,
    files: Vec<Box<File>>,
}

impl Directory {
    fn add_file(&mut self, file: File) {
        self.files.push(Box::new(file));
    }

    fn add_directory(&mut self, dir: Directory) {
        let mut new_dir = Box::new(dir);
        new_dir.parent = self as *const _;
        self.sub_directories.push(new_dir);
    }
}

#[derive(Debug, PartialEq)]
enum Command {
    NoOp,
    Cd,
    Ls,
}

type Result<T> = std::result::Result<T, String>;

fn main() {
    let lines = read_input_file("input.txt").unwrap();
    
    let root = parse_file_system(lines).unwrap();
       
    //print_file_system(&root);

    let directory_sizes = find_sizes_of_directories(&root);
    
    let smaller_than_10k = find_sub_directories_with_size_smaller_than_n(100000, directory_sizes.clone());

    println!("Part 1: {:?}", smaller_than_10k.iter().sum::<i32>());

    let used_space = directory_sizes[0];
    let smallest_to_delete = find_directory_to_free_n(directory_sizes, used_space, 30000000, 70000000);

    println!("Part 2: {:?}", smallest_to_delete);
}

fn parse_file_system(lines: Vec<String>) -> Result<Directory> {
    let mut root = Directory {
        name: "root".to_string(),
        parent: std::ptr::null(),
        sub_directories: Vec::new(),
        files: Vec::new(),
    };

    let mut current_command = Command::NoOp;

    let mut current_directory: &mut Directory = &mut root;

    for line in lines {
        if current_command == Command::Ls && !line.starts_with("$") {
            let file_or_dir = handle_ls_line(&line).unwrap();
            match file_or_dir {
                FileOrDirectory::File(file) => {
                    current_directory.add_file(*file);
                },
                FileOrDirectory::Directory(dir) => {
                    current_directory.add_directory(*dir);
                },
            }
            continue;
        }

        let mut parts = line.split_whitespace();

        parts.next();
        
        let command = parts.next().unwrap();

        current_command = parse_command(command).unwrap();

        match current_command {
            Command::Cd => {
                let dir = parts.next().unwrap();
                match dir {
                    ".." => {
                        current_directory = unsafe { &mut *(current_directory.parent as *mut Directory) };
                    },
                    "/" => {
                        current_directory = &mut root;
                    },
                    _ => {
                        let mut found = 0;
                        for (index, sub_dir) in current_directory.sub_directories.iter_mut().enumerate() {
                            if sub_dir.name == dir {
                                current_directory = &mut current_directory.sub_directories[index];
                                found = 1;
                                break;
                            }
                        }
                        if found == 0 {
                            panic!("Directory not found: {:?}", dir);
                        }
                    },
                }
            },
            Command::Ls => {
                continue;
            },
            _ => {
                println!("Unknown command: {:?}", current_command);
            },
        }
    }

    Ok(root)
}

fn handle_ls_line(str: &str) -> Result<FileOrDirectory> {
    let mut parts = str.split_whitespace();

    let first_word = parts.next().unwrap();

    if first_word == "dir" {
        let name = parts.next().unwrap();
        let dir = Directory {
            name: name.to_string(),
            parent: std::ptr::null(),
            sub_directories: Vec::new(),
            files: Vec::new(),
        };

        return Ok(FileOrDirectory::Directory(Box::new(dir)));
    }

    let size = first_word.parse::<i32>();
    let name = parts.next().unwrap();

    let file = File {
        name: name.to_string(),
        size: size.unwrap(),
    };

    Ok(FileOrDirectory::File(Box::new(file)))
}

fn parse_command(command: &str) -> Result<Command> {
    match command {
        "cd" => Ok(Command::Cd),
        "ls" => Ok(Command::Ls),
        _ => Err(format!("Unknown command: {}", command)),
    }
}

fn find_sub_directories_with_size_smaller_than_n(n: i32, sizes: Vec<i32>) -> Vec<i32> {
    let mut smaller_than_n = Vec::new();

    for size in sizes {
        if size < n {
            smaller_than_n.push(size);
        }
    }

    smaller_than_n
}

fn find_sizes_of_directories(root: &Directory) -> Vec<i32> {
    let mut stack = Vec::new();

    stack.push(root);

    let mut directory_sizes = Vec::new();

    while !stack.is_empty() {
        let current = stack.pop().unwrap();

        let max = std::i32::MAX;

        let size = find_directory_with_size(&current, max);

        directory_sizes.push(size);

        for dir in &current.sub_directories {
            stack.push(&dir);
        }
    }

    directory_sizes
}

fn find_directory_with_size(root: &Directory, size: i32) -> i32 {
    let mut total_size = 0;

    for file in &root.files {
        total_size += file.size;
    }

    for dir in &root.sub_directories {
        total_size += find_directory_with_size(&dir, size);
    }

    total_size
}

fn find_directory_to_free_n(sizes: Vec<i32>, used_space: i32, required_space: i32, total_space: i32) -> i32 {
    let mut smallest_to_delete = std::i32::MAX;

    for size in sizes {
        let space_available = total_space - used_space + size;

        if space_available >= required_space {
            if size < smallest_to_delete {
                smallest_to_delete = size;
            }    
        }
    }

    smallest_to_delete
}

fn print_file_system(root: &Directory) {
    println!("Directory: {:?}", root.name);

    for file in &root.files {
        println!("File: {:?} with size: {:?}", file.name, file.size);
    }

    for dir in &root.sub_directories {
        print_file_system(&dir);
    }
}
