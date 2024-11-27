use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};


fn main() {
    let matches = Command::new("ProtoBuf Processor")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Process input text files and serialize to protobuf format")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input-file")
                .value_name("INPUT")
                .help("Sets the input file")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output-file")
                .value_name("OUTPUT")
                .help("Sets the output file")
                .required(true),
        )
        .get_matches();

    //getting input file path and output file path from the clap.... 
    let inp_path = matches.get_one::<String>("input").unwrap();
    let out_path = matches.get_one::<String>("output").unwrap();

    if let Err(e) = process_files(inp_path, out_path) {
        eprintln!("Error: {}", e);
    }
    println!("Hooray!!!.....");
    println!("Program Run Successfully!!!! Go check your output file");

}
use prost::Message;
use std::io::BufWriter;

mod person {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated/_.rs"));


}

use person::Person;
fn encode_varint(value: u64) -> Vec<u8> {
    let mut buf = Vec::new();
    let mut value = value;
    while value > 127 {
        buf.push((value & 0x7f) as u8 | 0x80); 
        value >>= 7;
    }
    buf.push(value as u8); 
    buf
}


fn process_files(input: &str, output: &str) -> io::Result<()> {
    let input_file = File::open(input).expect("could not open input file");//open the input file 
    let output_file = File::create(output).expect("could not open output file");//creating a output file
    let mut writer = BufWriter::new(output_file);//creating a writing buffer later on used to write the data
    let reader = BufReader::new(input_file);//reading the input file using reader buffer

    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split(',').collect();

        if fields.len() != 3 {
            eprintln!("Invalid line format: {}", line);
            continue;
        }

        let person = Person {
            last_name: fields[0].to_string(),
            first_name: fields[1].to_string(),
            date_of_birth: fields[2].to_string(),
        };
        
        //encoding the person struct into byte
        let payload = person.encode_to_vec();
        println!("Serialized payload (bytes): {:?}", payload);

        let size_varint = encode_varint(payload.len() as u64);
        
        
        writer.write_all(&size_varint).expect("Error in write the byte size");
        writer.write_all(&payload).expect("Error in writing the payload");

    }

    Ok(())
}


//To Execute the code Run below command in terminal........
//cargo run --bin serializer -- -i input_file_path -o output_file_path

