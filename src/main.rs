use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::process;

pub fn read_csv<P: AsRef<Path>>(filename: P) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        match result {
            Ok(record) => println!("{:?}", record),
            Err(err) => {
                println!("error reading CSV from <stdin>: {}", err);
                process::exit(1);
            }
        }
    }

    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap::Command::new("CSV Reader")
        .version("1.0")
        .author("Gunner Clark")
        .about("Reads CSV files")
        .arg(
            clap::Arg::new("file")
                .help("Sets the input CSV file to use")
                .value_name("FILE")
                .value_parser(clap::value_parser!(String))
                .required(true)
                .index(1),
        )
        .get_matches();

    let filename = matches.get_one::<String>("file").unwrap();
    if Path::new(filename).exists() {
        println!("The file exists.");
        read_csv(filename)
    } else {
        println!("The file does not exist.");
        // main()
        Ok(())
    }
}
