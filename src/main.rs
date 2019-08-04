use clap::*;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() {
    let app = App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::with_name("pattern")
                .help("pattern to search")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("filename")
                .help("target file. use stdin if omitted")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("delimiter")
                .help("explicit delimiter pattern\n")
                .takes_value(true)
                .short("d")
                .long(
                    "delimiter\
                     ",
                )
                .env("LOGREP_DELIMITER")
                .default_value(r"^\s*$"),
        )
        .arg(
            Arg::with_name("regex")
                .help("search by regex")
                .short("r")
                .long("regex"),
        )
        .arg(
            Arg::with_name("ignore-case")
                .help("search case-insensitively")
                .short("i")
                .long("ignore-case"),
        )
        .arg(
            Arg::with_name("exclude")
                .help("exclude block if any of its lines matches pattern")
                .short("e")
                .long("exclude"),
        );
    let matches = app.get_matches();
    let delimiter = matches.value_of("delimiter").unwrap();
    let pattern = matches.value_of("pattern").unwrap();
    let use_regex = matches.is_present("regex");
    let case_insensitive = matches.is_present("ignore-case");
    let exclude = matches.is_present("exclude");
    let reader: Box<dyn BufRead> = match matches.value_of("filename") {
        None => {
            let stdin = std::io::stdin();
            let stdin = Box::leak(Box::new(stdin));
            Box::new(stdin.lock())
        }
        Some(filename) => File::open(filename)
            .map(|f| Box::new(BufReader::new(f)))
            .unwrap_or_else(|e| {
                eprintln!("IO Error: {}", e);
                std::process::exit(1)
            }),
    };

    let searcher =
        logrep::create_block_searcher(delimiter, pattern, use_regex, case_insensitive, exclude)
            .unwrap_or_else(|e| {
                eprintln!("Regex Parse Error: {}", e);
                std::process::exit(1);
            });

    let mut buf = String::new();
    let result = searcher
        .search_from_reader(reader, &mut buf)
        .unwrap_or_else(|e| {
            eprintln!("IO Error: {}", e);
            std::process::exit(1);
        });

    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    for line in result {
        writeln!(out, "{}", line).unwrap_or_else(|e| {
            eprintln!("IO Error: {}", e);
            std::process::exit(1)
        });
    }
}
