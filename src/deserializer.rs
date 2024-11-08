// important libs
use clap::Parser;
use protobuf::Message;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};

// importing module {person.rs} that is created by person.proto
mod person; 

// clap for cli
#[derive(Parser)]
#[command(name = "Person Deserializer")]
#[command(version = "1.0")]
#[command(author = "Shreyas K R <shreyas.kr@surya-soft.com>")]
#[command(about = "Deserializes Person data from Protocol Buffers format")]
struct Cli {
    #[arg(short, long, value_name = "INPUT", help = "Sets the input file to read")]
    input_file: String,

    #[arg(short, long, value_name = "OUTPUT", help = "Sets the output file to write")]
    output_file: String,
}

/// Decodes a u64 integer using variable-length encoding (varint format).
/// 
/// # Arguments
/// * `reader` - A mutable reference to an object implementing the `Read` trait, from which
///   the encoded integer will be read.
///
/// This function is used to extract the size of each serialized data entry, 
/// allowing us to allocate a buffer of the right size for deserialization.
fn varint_decode<R: Read>(reader: &mut R) -> io::Result<u64> {
    let mut value : u64 = 0;
    let mut shift = 0;

    // Loop through each byte to decode the varint-encoded size
    loop {
        // Read a single byte
        let byte = {
            let mut buf = [0u8; 1];
            reader.read_exact(&mut buf)?;
            buf[0]
        };
        // Combine byte with the current value
        value |= u64::from(byte & 0x7F) << shift;

        // Check if this byte is the last in the varint
        if byte & 0x80 == 0 {
            break;
        }

        shift += 7;
    }

    Ok(value)
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    
    // reading a file {serialized_data.bin}
    let input_file = File::open(&args.input_file)?;
    let mut reader = BufReader::new(input_file);

    // creating a output file {output.txt}
    let output_file = File::create(&args.output_file)?;
    let mut writer = BufWriter::new(output_file);

    // read and deserialize each person's entry
    while let Ok(size) = varint_decode(&mut reader) {
        // create a buffer of the appropriate size and read data into it
        let mut buffer = vec![0; size as usize];
        reader.read_exact(&mut buffer)?;

        // deserialize the buffer into a Person object
        let person = person::Person::parse_from_bytes(&buffer).expect("Failed to parse Person");

        // printing for coder understandibility
        println!("{:#?}", person);

        // write it into the file
        writeln!(
            writer,
            "{},{},{}",
            person.last_name,
            person.first_name,
            person.date_of_birth
        )?;
    }

    Ok(())
}
