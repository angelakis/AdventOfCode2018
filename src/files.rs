pub fn read_from_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::Read;
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Can't read file");
    contents
}
