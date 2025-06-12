use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::collections::HashMap;

mod HuffmanObjects;

use crate::HuffmanObjects::HuffmanObjects::HuffmanTree;

fn obtain_vocabulary(filepath: &String) -> Vec<u8>
{
    let mut buffer = Vec::new();
    let result = File::open(filepath)
        .and_then(|mut file| file.read_to_end(&mut buffer));

    match result 
    {
        Ok(_) => 
        {
            println!("File size: {} bytes", buffer.len());
            println!("File content in bits: {}",
                buffer.iter()
                    .map(|b| format!("{:08b}", b))
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        },
        Err(e) => 
        {
            eprintln!("Error reading file: {}", e);
        }
    }

    return buffer;
}

fn obtain_frequencies(vocabulary: &Vec<u8>) ->  HashMap<u8, usize>
{
    let mut frequencies: HashMap<u8, usize> = HashMap::new();

    for &word in vocabulary { *frequencies.entry(word).or_insert(0) += 1; }

    return frequencies;
}

fn main() -> io::Result<()>
{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2
    {
        eprintln!("No file determined. Use: {} <path_to_file>", args[0]);
        std::process::exit(1);
    }

    let filepath: &String = &args[1];
    let vocabulary: Vec<u8> = obtain_vocabulary(filepath);
    let frequencies: HashMap<u8, usize> = obtain_frequencies(&vocabulary);

    for (key, value) in &frequencies { println!("Byte {:08b} has {} appearances", key, value) };

    let mut freqTree = HuffmanTree::build_from_frequencies(&frequencies);

    

    return Ok(());
}