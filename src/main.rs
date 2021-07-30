#![feature(array_chunks)]

mod printlns;

fn main() {
    let args = get_args();

    if args.len() == 0 {
        print_help()
    } else {
        let switch = args[0].clone();
        let files = remove_first(args);
        execute(&switch, files);
        
    }
}

fn execute(switch: &str, files: Vec<String>) {
    match switch {
        "-e" | "--encode" => { encode(files); }
        "-d" | "--decode" => { decode(files); }
        "-h" | "--help"   => { print_help(); }
        _                 => {
            println!("Unknown switch: `{}`", switch);
            print_help();
        }
    }
}

fn print_help() {
    use printlns::printlns;

    printlns!(
        "Switches:"
        "--help   | -h : display this help menu"
        "--encode | -e : text -> bzzz conversion"
        "--decode | -d : bzzz -> text conversion"
        "After the -e or -d switches, any number of filenames may be"
        "provided. If none are provided, input will be taken from stdin."
    );
}

fn encode(args: Vec<String>) {
    if args.len() == 0 {
        encode_stdin();
    } else {
        encode_files(args);
    }
}

fn decode(args: Vec<String>) {
    if args.len() == 0 {
        decode_stdin();
    } else {
        decode_files(args);
    }
}

fn encode_stdin() {
    let lines = get_stdin();
    encode_lines(lines);
}

fn encode_files(files: Vec<String>) {
    for file in &files {
        if !file_exists(file) {
            eprintln!("File `{}` does not exist.", &file);
            return;
        }
    }

    for file in &files {
        let lines = get_lines_from_file(file);
        encode_lines(lines);
    }
}

fn decode_stdin() {
    let lines = get_stdin();
    decode_lines(lines);
}

fn decode_files(files: Vec<String>) {
    for file in &files {
        if !file_exists(file) {
            eprintln!("File `{}` does not exist.", &file);
            return;
        }
    }

    for file in &files {
        let lines = get_lines_from_file(file);
        decode_lines(lines);
    }
}

fn encode_lines(lines: Vec<String>) {
    for line in lines {
        let encoded_line = encode_line(line);
        println!("{}", encoded_line);
    }
}

fn decode_lines(lines: Vec<String>) {
    for line in &lines {
        if !is_valid_input(line) {
            eprintln!("Invalid input:\n{}", &line);
            return;
        }
    }

    for line in lines {
        decode_line(&line);
    }
}

fn encode_line(line: String) -> String {
    let mut nybbles = vec![];

    for c in line.bytes() {
        let mut upper = "bz".to_string();
        let mut lower = "bz".to_string();

        upper.push_str(&"z".repeat(((0xf0 & c) >> 4) as usize));
        lower.push_str(&"z".repeat((0x0f & c) as usize));

        nybbles.push(upper);
        nybbles.push(lower);
    }

    return nybbles.join(" ");
}

fn decode_line(line: &str) {
    use std::convert::TryInto;

    println!(
        "{}",
        std::str::from_utf8(
            &line
                .split(" ")
                .map(|x| x.len() - 2)
                .map(|x| x.try_into())
                .map(|x| x.unwrap_or(0))
                .collect::<Vec<u8>>()
                .array_chunks::<2>()
                .map(|[h, l]| h << 4 | l)
                .collect::<Vec<u8>>()
        ).unwrap()
    );
}

fn remove_first<T>(vec: Vec<T>) -> Vec<T> {
    let mut temp = vec.into_iter();
    temp.next();
    return temp.collect();
}

fn get_lines_from_file(file: &str) -> Vec<String> {
    let text = read_file(file);
    return get_lines_from_text(&text);
}

fn get_lines_from_text(text: &str) -> Vec<String> {
    return text
        .lines()
        .map(|x| x.to_string())
        .collect();
}

fn read_file(file: &str) -> String {
    use std::fs::read;

    return String::from_utf8(
            read(file).expect(
                &format!("reading file {}", file),
            )
        )
        .expect(
            &format!("converting bytes from {} to utf8 text", file),
        );
}

fn get_args() -> Vec<String> {
    use std::env::args;

    let mut args = args();
    args.next();
    return args.collect();
}

fn get_stdin() -> Vec<String> {
    use std::io::{BufRead, stdin};

    return stdin()
        .lock()
        .lines()
        .map(|x| x.expect("reading from stdin"))
        .collect();
}

fn is_valid_input(input: &str) -> bool {
    use regex::Regex;

    let pattern = Regex::new("^(?:bz{1,16} bz{1,16}(?: bz{1,16} bz{1,16})*)?$")
        .unwrap();

    return pattern.is_match(input);
}

fn file_exists(file: &str) -> bool {
    use std::path::Path;

    return Path::new(&file).is_file();
}
