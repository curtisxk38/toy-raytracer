use std::fs;

pub fn parse(filename: &String) {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    for line in contents.lines() {
        if line != "" {
            println!("{:?}", line);
            let words: Vec<&str> = line.split(" ").collect();
        }
    }
}
