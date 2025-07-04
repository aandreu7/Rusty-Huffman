use std::io::{self};
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use fnv::FnvHasher;

mod HuffmanObjects;
use crate::HuffmanObjects::huffman_encoding;
use crate::HuffmanObjects::huffman_decoding;

pub mod EnvHandling;

pub type DetHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FnvHasher>>;

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
        None => { std::process::exit(1); }
    }
}