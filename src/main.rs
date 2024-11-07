use clap::Parser;
use person::Person;
use protobuf::Message;
use std::io::{self, BufRead, Write,};
use std::fs::File;
use std::path::Path;

mod person;

#[derive(Parser)]
#[command(name = "Person Serializer")]
#[command(version = "1.0")]
#[command(author = "Shreyas <shreyas.kr@surya-soft.com@example.com>")]
#[command(about = "Serializes Person data into Protocol Buffers format")]
struct Cli{
    #[arg(short = 'i', long = "input-file", value_name = "INPUT", help = "Sets the input file to read")]
    input_file: String,
    
    #[arg(short = 'o', long = "output-file", value_name = "OUTPUT", help = "Sets the output file to write")]
    output_file: String,
}

fn variant_encode(mut value: u64, buf: &mut Vec<u8>) {
    while value > 127 {
        buf.push((value & 0x7F) as u8 | 0x80);
        value >>= 7;
    }
    buf.push(value as u8);
}
fn main() -> io::Result<()>{

    let args= Cli::parse();

    let file = File::open(&args.input_file)?;
    let reader = io::BufReader::new(file);

    let mut output = File::create(&args.output_file)?;

    for line in reader.lines(){
        let line = line?;
        let fields : Vec<&str> = line.split(',').collect();

        if fields.len() != 3 {
            eprintln!("Skipping invalid line : {}", line);
            continue;
        }

        let mut person = person::Person::new();
        person.set_last_name(fields[0].to_string());
        person.set_first_name(fields[1].to_string());
        person.set_date_of_birth(fields[2].to_string());


        let serialized_data = person.write_to_bytes().unwrap();

        let size = serialized_data.len() as u64;

        let mut size_varint: Vec<u8> = Vec::new();

        variant_encode(size, &mut size_varint);

        output.write_all(&size_varint)?;
        output.write_all(&serialized_data)?;   
    }

    Ok(())
}
