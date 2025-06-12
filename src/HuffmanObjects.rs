pub mod HuffmanObjects 
{
    use std::collections::BinaryHeap;
    use std::collections::HashMap;
    use std::cmp::Ordering;

    #[derive(PartialEq, Eq)]
    enum HuffmanNode
    {
        Leaf { byte: u8, freq: usize },
        Internal { internalFreq: usize, left: Box<HuffmanNode>, right: Box<HuffmanNode> },
    }

    #[derive(Eq, PartialEq)]
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
            let mut tree = HuffmanTree{ tree: BinaryHeap::<HuffmanTreeItem>::new(), nodes: Vec::<Box<HuffmanNode>>::new(), };

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
            
            return tree;
        }
    }

    pub struct HuffmanTree
    {
        tree: BinaryHeap<HuffmanTreeItem>,
        nodes: Vec<Box<HuffmanNode>>,
    }
}
