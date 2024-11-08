// important libs
use clap::Parser;
use protobuf::Message;
use std::io::{self, BufRead, Write,};
use std::fs::File;

// importing module {person.rs} that is created by person.proto
mod person;

// clap for cli
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



/// Encodes a u64 integer using variable-length encoding (varint format) and appends it to a buffer.
/// 
/// # Arguments
/// * `value` - The integer to encode.
/// * `buf` - A mutable reference to the buffer where the encoded integer will be appended.
///
/// This function is used to prepend each serialized data entry with its size, allowing
/// for easier parsing during deserialization.
fn variant_encode(mut value: u64, buf: &mut Vec<u8>) {
    while value > 127 {
        buf.push((value & 0x7F) as u8 | 0x80);
        value >>= 7;
    }
    buf.push(value as u8);
}
fn main() -> io::Result<()>{

    let args= Cli::parse();

    // reading a txt file {person.txt}
    let file = File::open(&args.input_file)?;
    let reader = io::BufReader::new(file);

    //creating a output file {serialized_data.bin}
    let mut output = File::create(&args.output_file)?;

    //reading each line from input file
    for line in reader.lines(){
        let line = line?; // to check if line is in proper format
        let fields : Vec<&str> = line.split(',').collect(); // to split each value from each row

        if fields.len() != 3 { // if the row doesnot have 3 values then its in wrong format so it skips
            eprintln!("Skipping invalid line : {}", line);
            continue;
        }

        // giving those values to person struct
        let mut person = person::Person::new();
        person.set_last_name(fields[0].to_string());
        person.set_first_name(fields[1].to_string());
        person.set_date_of_birth(fields[2].to_string());

        // serializing the person data
        let serialized_data = person.write_to_bytes().unwrap();
        
        let size = serialized_data.len() as u64;

        // printing for coder understandability
        println!("{:?}\nsize ==> {}", serialized_data, size);
        
        let mut size_varint: Vec<u8> = Vec::new();

        variant_encode(size, &mut size_varint); // calling variant_encode to get encoded size

        // writing those into the file
        output.write_all(&size_varint)?;
        output.write_all(&serialized_data)?;   
    }

    Ok(())
}
