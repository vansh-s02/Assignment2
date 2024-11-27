use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use prost::Message;

mod person {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated/_.rs"));
}
use person::Person;

fn main() {
    let matches = Command::new("ProtoBuf Processor")
        .version("1.0")
        .author("Vansh Singhal")
        .about("Process input protobuf file and write readable output")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input-file")
                .value_name("INPUT")
                .help("Sets the input protobuf file")
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

    process_files(inp_path, out_path)//calling the function to process fi;e
}


fn decode_varint<R: Read>(reader: &mut R) -> io::Result<Option<u64>> {
    let mut value = 0u64;
    let mut shift = 0;

    loop {
        let mut byte = [0u8; 1]; // Reading one byte at a time
        let bytes_read = reader.read(&mut byte).expect("Error in reading the byte from the reader");

        // condition for checking termination case(End of file)
        if bytes_read == 0 {
            return Ok(None); // End of file reach
        }

        let byte = byte[0]; // getting the byte value

        value |= ((byte & 0x7F) as u64) << shift;
        if (byte & 0x80) == 0 {
            break;
        }
        shift += 7;

        // condition for preventing corrupted data...so it not get into infinte loop.
        if shift >= 64 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "varint too large",
            ));
        }
    }

    Ok(Some(value))
}

fn process_files(input: &str, output: &str) {

    let input_file = File::open(input).expect("could not open input file");//open the input file 
    let output_file = File::create(output).expect("could not open output file");//creating a output file
    let mut writer = BufWriter::new(output_file);//creating a writing buffer later on used to write the data
    let mut reader = BufReader::new(input_file);//reading the input file using reader buffer

    
    loop {
        match decode_varint(&mut reader) {
            //getting some data while reading so decode them and write it to output file
            Ok(Some(size_length)) => {
                let mut buf = vec![0u8; size_length as usize];
                reader.read_exact(&mut buf).expect("Failed to read message");

                //storing the data from reader in Person struct format....
                let person = Person::decode(&*buf).expect("Failed to decode protobuf payload");

                //writing the person struct into the output file
                writeln!(
                    writer,
                    "{}, {}, {}",
                    person.last_name, person.first_name, person.date_of_birth
                )
                .expect("Failed to write to output file");
            }
            //end of file reach if there is no data to read and write
            Ok(None) => {
                println!("End of file reached.Go Check your output file");
                break;
            }
            //throw an error if getting an issue....
            Err(e) => {
                eprintln!("Error decoding varint: {}", e);
                break;
            }
        }
    }

}
//To Execute the code Run below command in terminal........
// cargo run --bin deserializer -- -i input_file_path -o output_file_path
