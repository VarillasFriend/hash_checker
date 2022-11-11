use sha2::{Sha256, Digest};
use std::{fs, io, thread, path::{Path, PathBuf}, sync::mpsc};

/// Generates a Sha256 hash for a given file and returns it as a String
fn generate_hash_for_file(path: &Path) -> String {
    let mut file = fs::File::open(&path)
        .expect("Unable to read file");

    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher).expect("Error hashing");

    format!("{:x}", hasher.finalize())
}

/// Generates hashes for each file inside a folder and returns a vector with them
fn generate_hashes_for_folder(path: PathBuf) -> Vec<String> {
    let mut vector: Vec<String> = Vec::new();

    for entry in fs::read_dir(path.as_path()).expect("Unable to list files") {
        let entry = entry.expect("Unable to get entry");
        println!("Processing: {}", entry.path().display());

        if !entry.file_type().expect("Could not resolve file type").is_dir() {
            vector.push(generate_hash_for_file(&entry.path()));
        } 
    }

    vector
}

/// Checks that every single element of hashes1 is in hashes2 as well
/// Returns true if that's the case
fn compare_hash_vectors(hashes1: Vec<String>, hashes2: Vec<String>) -> bool {
    for hash in hashes1 {
        if !hashes2.iter().any(|x| x == &hash) {
            return false
        }
    }

    true
}

/// Generates hashes for two folders on different threads and sends the resulting
/// vectors to compare_hash_vectors to check if folder1 has the same elements as
/// folder2
pub fn compare_folders(folder1: PathBuf, folder2: PathBuf) -> bool {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
        let hashes1 = generate_hashes_for_folder(folder1);
        tx1.send(hashes1).unwrap();
    });

    thread::spawn(move || {
        let hashes2 = generate_hashes_for_folder(folder2);
        tx2.send(hashes2).unwrap();
    });

    let received1 = rx1.recv().unwrap();
    let received2 = rx2.recv().unwrap();

    compare_hash_vectors(received1, received2)
}

/// Gets name of a file without extensions and returns it as a String
fn get_name(path: PathBuf) -> String {
    let mut name = String::new();
    let full_name = path
        .file_name()
        .expect("Could not resolve file name")
        .to_str()
        .expect("Could not resolve file name");
    let name_chars = full_name.char_indices();

    for char_index in name_chars {
        if char_index.1 == '.' && char_index.0 != 0 {
            break;
        }

        name.push(char_index.1)
    }

    name
}

/// Gets all files from folders and returns them as a Vector 
fn get_paths_from_folder(path: PathBuf) -> Vec<(PathBuf, String)> {
    let mut vector: Vec<(PathBuf, String)> = Vec::new();

    for entry in fs::read_dir(path.as_path()).expect("Unable to list files") {
        let entry = entry.expect("Unable to get entry");

        if !entry.file_type().expect("Could not resolve file type").is_dir() {
            vector.push((entry.path(), get_name(entry.path())));
        } 
    }

    vector
}

/// If a file is not present in the second folder, it gets copied to Lost to be
/// recovered
fn recover(file: PathBuf) {
    let path = Path::new("./Lost/").join(file
        .file_name()
        .expect("Could not read file name")
        .to_str()
        .expect("Could not read file name"));

    fs::File::create(&path).expect("Couldn't create file");

    fs::copy(file, path).expect("Could not copy file");
}

/// Checks that every file present in folder1 is also present in folder2 and calls
/// recover() on the files that are not
fn compare_paths_vectors(folder1: Vec<(PathBuf, String)>, folder2: Vec<(PathBuf, String)>) -> bool {
    let mut has_same_elements = true;
    for file in folder1 {
        if !folder2.iter().any(|x| x.1 == file.1) {
            println!("File missing: {}", file.0.display());
            recover(file.0);
            has_same_elements = false
        }
    }

    has_same_elements
}

/// Compares files in each folders with the names of the files (without extensions)
/// instead of using hashes
pub fn compare_folders_with_names(folder1: PathBuf, folder2: PathBuf) -> bool {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
        let names1 = get_paths_from_folder(folder1);
        tx1.send(names1).unwrap();
    });

    thread::spawn(move || {
        let names2 = get_paths_from_folder(folder2);
        tx2.send(names2).unwrap();
    });
    
    let received1 = rx1.recv().unwrap();
    let received2 = rx2.recv().unwrap();

    compare_paths_vectors(received1, received2)
}
