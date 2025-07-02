use std::io::{self};
use std::collections::HashMap;

mod HuffmanObjects;
use crate::HuffmanObjects::huffman_encoding;
use crate::HuffmanObjects::huffman_decoding;

pub mod EnvHandling;

fn main() -> io::Result<()>
{
    match EnvHandling::check_entry()
    {
        Some((mode, filepath)) => 
        {
            if mode == "-e" { huffman_encoding(&filepath); }
            else if mode == "-d" { huffman_decoding(&filepath); }
            else { std::process::exit(1); }
            return Ok(());
        }
        None => 
        {
            std::process::exit(1);
        }
    }
}