use std::env;
use std::fs::File;
use std::io::{self, Read, BufReader, Write, BufWriter};

use crate::DetHashMap;

pub fn obtain_vocabulary(filepath: &str) -> Vec<u8>
{
    let mut buffer = Vec::new();
    let result = File::open(filepath)
        .and_then(|mut file| file.read_to_end(&mut buffer));

    match result 
    {
        Ok(_) => 
        {
            //println!("File size: {} bytes", buffer.len());
            /*
            println!("File content in bits: {}",
                buffer.iter()
                    .map(|b| format!("{:08b}", b))
                    .collect::<Vec<_>>()
                    .join(" ")
            );
            */
        },
        Err(e) => 
        {
            eprintln!("Error reading file: {}", e);
        }
    }

    return buffer;
}

pub fn obtain_frequencies(vocabulary: &Vec<u8>) ->  DetHashMap<u8, usize>
{
    let mut frequencies: DetHashMap<u8, usize> = DetHashMap::default();

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

pub fn write_encoded_file(filename: &str, codes: &DetHashMap<u8, Vec<bool>>, encoded_data: &[u8], original_len: usize) -> std::io::Result<(String)> 
{
    let full_path = format!("{}{}", filename, ".huff");
    let mut file = BufWriter::new(File::create(&full_path)?);

    // 1. Writes number of codes (u16)
    let codes_count = codes.len() as u16;
    file.write_all(&codes_count.to_be_bytes())?;

    // 2. Writes, for each code:
    //    - 1 byte: character
    //    - 1 byte: length in bits
    //    - N bytes: character bit-coded
    for (&byte, code_bits) in codes.iter()
    {
        file.write_all(&[byte])?; // character

        let code_len = code_bits.len() as u8;
        file.write_all(&[code_len])?; // length in bits

        let mut byte_buffer = 0u8;
        let mut bits_in_buffer = 0;

        for &bit in code_bits
        {
            byte_buffer <<= 1;
            if bit { byte_buffer |= 1; }
            bits_in_buffer += 1;
            if bits_in_buffer == 8
            {
                file.write_all(&[byte_buffer])?; // character bit-coded
                byte_buffer = 0;
                bits_in_buffer = 0;
            }
        }

        // If bits are left to complete a byte, stuff it whit 0s at most-significant positions
        if bits_in_buffer > 0
        {
            byte_buffer <<= 8 - bits_in_buffer;
            file.write_all(&[byte_buffer])?; // character bit-coded
        }
    }

    // 3. Writes original length (u32)
    file.write_all(&(original_len as u32).to_be_bytes())?;

    // 4. Writes codified data
    file.write_all(encoded_data)?;

    return Ok((full_path));
}

pub fn read_encoded_file(filename: &str) -> std::io::Result<(DetHashMap<u8, Vec<bool>>, Vec<u8>, usize)>
{
    let mut file = BufReader::new(File::open(filename)?);

    // 1. Reads number of codes (u16)
    let mut buffer2 = [0u8; 2];
    file.read_exact(&mut buffer2)?; // Reads two bytes
    let codes_count = u16::from_be_bytes(buffer2);

    // 2. Reads, for each code:
    //    - 1 byte: character
    //    - 1 byte: length in bits
    //    - N bytes: character bit-coded
    let mut codes: DetHashMap<u8, Vec<bool>> = DetHashMap::default();
    for _ in 0..codes_count
    {
        let mut byte_buf = [0u8; 1];
        file.read_exact(&mut byte_buf)?; // character
        let byte: u8 = byte_buf[0];

        let mut len_buf = [0u8; 1];
        file.read_exact(&mut len_buf)?; // length in bits
        let code_len = len_buf[0] as usize;

        // It reads bytes, but works with bits, so conversion needed
        let bytes_needed = (code_len + 7) / 8;
        let mut code_bytes = vec![0u8; bytes_needed];
        file.read_exact(&mut code_bytes)?; // characters bit-coded

        // Converts from bytes to bits (only code_len bits)
        let mut code_bits: Vec<bool> = Vec::with_capacity(code_len);
        for i in 0..code_len
        {
            let byte_index = i / 8;
            let bit_index = 7 - (i % 8);
            let bit = (code_bytes[byte_index] >> bit_index) & 1 == 1;   // Locates searched bit at byte's least-significant position, 
                                                                        // so as to get its value
            code_bits.push(bit);
        }

        codes.insert(byte, code_bits);
    }

    // 3. Reads original length (u32)
    let mut len_buf4 = [0u8; 4]; // 4 bytes needed to read a u32 number
    file.read_exact(&mut len_buf4)?;
    let original_len = u32::from_be_bytes(len_buf4) as usize;

    // 4. Reads codified data
    let mut encoded_data = Vec::new();
    file.read_to_end(&mut encoded_data)?;

    return Ok((codes, encoded_data, original_len));
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