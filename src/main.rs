use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::collections::HashMap;

mod HuffmanObjects;
use crate::HuffmanObjects::HuffmanObjects::HuffmanTree;

mod EnvHandling;
use crate::EnvHandling::EnvHandling::obtain_vocabulary;
use crate::EnvHandling::EnvHandling::obtain_frequencies;
use crate::EnvHandling::EnvHandling::check_entry;
use crate::EnvHandling::EnvHandling::write_encoded_file;

fn exec_huffman(filepath: &String)
{
    let vocabulary: Vec<u8> = obtain_vocabulary(filepath);
    let frequencies: HashMap<u8, usize> = obtain_frequencies(&vocabulary);

    for (key, value) in &frequencies { println!("Byte {:08b} has {} appearances", key, value) };

    let mut freqTree: HuffmanTree = HuffmanTree::build_from_frequencies(&frequencies);

    let encoded_data: Vec<u8> = freqTree.encode_data(&vocabulary);

    write_encoded_file(filepath, )
}

fn main() -> io::Result<()>
{
    match check_entry()
    {
        Some(filepath) => 
        {
            exec_huffman(&filepath);
            return Ok(());
        }
        None => 
        {
            std::process::exit(1);
        }
    }
}