use std::env;
use std::fs::File;
use std::io::{self, Read, BufReader, Write, BufWriter};
use std::collections::HashMap;

pub fn obtain_vocabulary(filepath: &str) -> Vec<u8>
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

pub fn obtain_frequencies(vocabulary: &Vec<u8>) ->  HashMap<u8, usize>
{
    let mut frequencies: HashMap<u8, usize> = HashMap::new();

    for &word in vocabulary { *frequencies.entry(word).or_insert(0) += 1; }

    return frequencies;
}

pub fn write_decoded_file(filename: &str, decoded_data: &[u8]) -> std::io::Result<()>
{
    let full_path = format!("{}{}", filename, ".decoded");
    let mut file = File::create(full_path)?;
    file.write_all(decoded_data);
    return Ok(());
}

pub fn write_encoded_file(filename: &str, frequencies: &HashMap<u8, usize>, encoded_data: &[u8], original_len: usize) -> std::io::Result<()> 
{
    let full_path = format!("{}{}", filename, ".huff");
    let mut file = BufWriter::new(File::create(full_path)?);

    // Writes number of entries
    let entry_count = frequencies.len() as u16;
    file.write_all(&entry_count.to_be_bytes())?;

    // Writes frequencies dict: (byte, frequency)
    for (&byte, &freq) in frequencies.iter()
    {
        file.write_all(&[byte])?;
        file.write_all(&(freq as u32).to_be_bytes())?;
    }

    // Writes length of original message
    file.write_all(&(original_len as u32).to_be_bytes())?;

    // Writes codified data
    file.write_all(encoded_data)?;

    return Ok(());
}

pub fn read_encoded_file(filename: &str) -> std::io::Result<(HashMap<u8, usize>, Vec<u8>, usize)>
{
    let mut file = BufReader::new(File::open(filename)?);
    let mut buffer = [0u8; 2];

    // Reads number of entries
    file.read_exact(&mut buffer)?;
    let entry_count = u16::from_be_bytes(buffer);

    // Reads frequencies dict: (byte, frequency)
    let mut frequencies: HashMap<u8, usize> = HashMap::new();
    for _ in 0..entry_count
    {
        let mut byte_buf = [0u8; 1];
        let mut freq_buf = [0u8; 4];
        file.read_exact(&mut byte_buf)?;
        file.read_exact(&mut freq_buf)?;
        let byte = byte_buf[0];
        let freq = u32::from_be_bytes(freq_buf) as usize;
        frequencies.insert(byte, freq);
    }

    // Reads length of original message
    let mut len_buf = [0u8; 4];
    file.read_exact(&mut len_buf)?;
    let original_len: usize = u32::from_be_bytes(len_buf) as usize;

    // Reads codified data
    let mut encoded_data: Vec<u8> = Vec::new();
    file.read_to_end(&mut encoded_data);

    return Ok((frequencies, encoded_data, original_len));
}

pub fn check_entry() -> Option<(String, String)> 
{
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 
    {
        eprintln!("Incorrect use. Sintax: {} [-e|-d] <path to file>", args[0]);
        return None;
    }

    let mode = &args[1];
    let filepath = &args[2];

    if mode != "-e" && mode != "-d" 
    {
        eprintln!("First param must be either -e (encode) or -d (decode).");
        return None;
    }

    Some((mode.clone(), filepath.clone()))
}