pub mod EnvHandling
{
    use std::env;
    use std::fs::File;
    use std::io::{self, Read, Write};
    use std::collections::HashMap;

    pub fn obtain_vocabulary(filepath: &String) -> Vec<u8>
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

    pub fn write_encoded_file(filename: &str, data: &[u8]) -> std::io::Result<()> 
    {
        let mut file = File::create(filename)?;
        file.write_all(data)?;

        return Ok(());
    }

    pub fn check_entry() -> Option<String>
    {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2
        {
            eprintln!("No file determined. Use: {} <path_to_file>", args[0]);
            return None;
        }

        return Some(args[1].clone());
    }
}