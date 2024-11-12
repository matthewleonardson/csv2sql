use std::{env, error::Error, ffi::OsString, fs::File, io::Write, process};

use csv::Reader;

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;

    let mut rdr = csv::Reader::from_reader(file);
    let _ = write_sql_file("schema.sql", &mut rdr);

    Ok(())

}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {

    match env::args_os().nth(1) {
        None => Err(From::from("expected an argument, got none")),
        Some(file_path) => Ok(file_path),
    }

}

fn write_sql_file(filename: &str, reader: &mut Reader<File>) -> Result<(), Box<dyn Error>> {

    let mut file = File::create(filename)?;

    let mut table_creation_string = String::from("CREATE TABLE table (id SERIAL PRIMARY KEY,");

    if let Ok(header) = reader.headers() {
        for h in header {
            table_creation_string.push_str(&format!("{} VARCHAR(50),", h));
        }
    }

    // replace final comma with a parentheses and semicolon
    table_creation_string.pop();
    table_creation_string.push(')');
    table_creation_string.push(';');

    // write to the sql file
    writeln!(file, "{}", table_creation_string)?;

    for result in reader.records() {
        let record = result?;

        let mut insertion_string = String::from("INSERT INTO table VALUES (DEFAULT, ");

        for (i, value) in record.iter().enumerate() {

            insertion_string.push_str(&format!("'{}'", value.replace("'", "''")));

            if i < record.len() - 1 {
                insertion_string.push_str(", ");
            }

        }

        insertion_string.push_str(");"); 
        writeln!(file, "{}", insertion_string)?; 

    }

    Ok(())

}

fn main() {

    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }

}