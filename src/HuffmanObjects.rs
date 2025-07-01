pub mod HuffmanObjects 
{
    use std::collections::BinaryHeap;
    use std::collections::HashMap;
    use std::cmp::Ordering;
    use std::fmt;

    #[derive(PartialEq, Eq)]
    pub enum HuffmanNode
    {
        Leaf { byte: u8, freq: usize },
        Internal { internalFreq: usize, left: Box<HuffmanNode>, right: Box<HuffmanNode> },
    }

    impl fmt::Display for HuffmanNode
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
        {
            fn fmt_rec(node: &HuffmanNode, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result
            {
                let indent = " ".repeat(depth);
                match node
                {
                    HuffmanNode::Leaf { byte, freq } => { return writeln!(f, "{}Leaf(byte: {:08b}, freq: {})", indent, byte, freq); }
                    HuffmanNode::Internal { internalFreq, left, right } =>
                    {
                        writeln!(f, "{}Internal(freq: {})", indent, internalFreq)?;
                        fmt_rec(left, f, depth + 1)?;
                        return fmt_rec(right, f, depth + 1);
                    }
                }
            }
            return fmt_rec(self, f, 0);
        }
    }

    #[derive(PartialEq, Eq)]
    struct HuffmanTreeItem(pub usize, pub Box<HuffmanNode>);

    impl Ord for HuffmanTreeItem
    {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering 
        { 
            other.0.cmp(&self.0) 
        }
    }

    impl PartialOrd for HuffmanTreeItem
    {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
        {
            Some(self.cmp(other))
        }
    }

    impl HuffmanTree
    {
        pub fn build_from_frequencies(frequencies: &HashMap<u8, usize>) -> Self 
        {
            let mut tree = HuffmanTree{ root: None, tree: BinaryHeap::<HuffmanTreeItem>::new(), nodes: Vec::<Box<HuffmanNode>>::new(), };

            // Step 1: Adds leaf nodes into the heap
            for (&byte, &freq) in frequencies.iter() 
            {
                let leaf = Box::new(HuffmanNode::Leaf { byte, freq });
                tree.tree.push(HuffmanTreeItem(freq, leaf));
            }

            // Step 2: Builds tree combining nodes
            while tree.tree.len() > 1 
            {
                // Extracts nodes with lowest frequencies
                let HuffmanTreeItem(freq1, left) = tree.tree.pop().unwrap();
                let HuffmanTreeItem(freq2, right) = tree.tree.pop().unwrap();

                // Creates intern node with the sum of frequencies
                let internal_freq = freq1 + freq2;
                let internal_node = Box::new
                (
                    HuffmanNode::Internal 
                    {
                        internalFreq: internal_freq,
                        left,
                        right,
                    }
                );

                // Inserts new intern node
                tree.tree.push(HuffmanTreeItem(internal_freq, internal_node));
            }
            
            let HuffmanTreeItem(_freq, root_node) = tree.tree.pop().unwrap();
            tree.root = Some(root_node);

            return tree;
        }

        fn generate_codes(&self) -> HashMap<u8, Vec<bool>> 
        {
            let mut codes = HashMap::new();

            fn traverse(node: &HuffmanNode, prefix: Vec<bool>, codes: &mut HashMap<u8, Vec<bool>>) 
            {
                match node 
                {
                    HuffmanNode::Leaf { byte, .. } => { codes.insert(*byte, prefix); },
                    HuffmanNode::Internal { left, right, .. } => 
                    {
                        let mut left_prefix = prefix.clone();
                        left_prefix.push(false);
                        traverse(left, left_prefix, codes);

                        let mut right_prefix = prefix;
                        right_prefix.push(true);
                        traverse(right, right_prefix, codes);
                    }
                }
            }

            if let Some(root) = &self.root { traverse(root, Vec::new(), &mut codes); }

            return codes;
        }

        pub fn encode_data(&self, vocabulary: &Vec<u8>) -> Vec<u8> 
        {
            let codes: HashMap<u8, Vec<bool>> = self.generate_codes();

            let mut bit_buffer: Vec<bool> = Vec::new();

            // 1. For each file's byte, overwrittes the original byte with its corresponding code (sequence of bits, Variable Length Coding)
            for &byte in vocabulary 
            {
                // 2. Adds code (sequence of bits) from byte to buffer
                if let Some(code) = codes.get(&byte) { bit_buffer.extend(code); } 
                else { panic!("Byte with no Huffman code: {}", byte); }
            }

            // 3. Converts sequence of bits to real bytes (u8):
            // It is not possible to store sequences of bits in a file, they have to be converted into bytes (u8) previously
            let mut encoded_bytes: Vec<u8> = Vec::new();
            let mut current_byte: u8 = 0;
            let mut bits_in_current_byte = 0;

            for bit in bit_buffer 
            {
                current_byte <<= 1;           // Left shift to add new bit
                if bit { current_byte |= 1; } // If bit is 1, puts it on least significant position. Else, let it as zero
                bits_in_current_byte += 1;

                if bits_in_current_byte == 8 
                {
                    encoded_bytes.push(current_byte);
                    current_byte = 0;
                    bits_in_current_byte = 0;
                }
            }

            // If bits are left (less than 8) in the last byte, stuffs last byte with zeros
            if bits_in_current_byte > 0 
            {
                current_byte <<= 8 - bits_in_current_byte;
                encoded_bytes.push(current_byte);
            }

            return encoded_bytes;
        }

        pub fn print(&self)
        {
            if let Some(node) = &self.root { println!("{}", node); }
            else { println!("Empty tree."); }
        }
    }

    pub struct HuffmanTree
    {
        root: Option<Box<HuffmanNode>>,
        tree: BinaryHeap<HuffmanTreeItem>,
        nodes: Vec<Box<HuffmanNode>>,
    }
}
