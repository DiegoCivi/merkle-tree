use std::hash::{DefaultHasher, Hash, Hasher};

struct MerkleTree {
    arr: Vec<Vec<u64>>,
}

// TODO: Check if this function should be inside the impl
fn calculate_elements_hashes<T: Hash>(elements: &Vec<T>) -> Vec<u64> {
    elements.iter().map(|elem| {
        let mut hasher = DefaultHasher::new();
        elem.hash(&mut hasher);
        hasher.finish()
    }).collect()
}

// TODO: Check if this function should be inside the impl
fn add_remaining_hashes(hashed_elements: Vec<u64>) -> Vec<Vec<u64>> {
    let mut arr = Vec::new();
    arr.push(hashed_elements.clone());
    let mut hashes = hashed_elements;
    while hashes.len() != 1 {
        // TODO: Check if this way of concatenating the hashes is correct
        hashes = hashes.chunks(2).map(|chunk| {
            let concatenated = chunk[0].to_string() + &chunk[1].to_string();
            let mut hasher = DefaultHasher::new();
            concatenated.hash(&mut hasher);
            let temp = hasher.finish();
            println!("({:?} + {:?}) = {:?}", chunk[0], chunk[1], temp);
            println!("****************");
            temp
        }).collect();
        arr.push(hashes.clone());
    }

    arr
}

impl MerkleTree {

    fn new<T: Hash>(elements: &Vec<T>) -> Self {
        // TODO: Add support for vectors that have a len that is not a power of 2.
        // Hash every element of the array
        let hashed_elements = calculate_elements_hashes(elements);
        let arr = add_remaining_hashes(hashed_elements);
        
        Self { arr }
    }

    fn print_arr(&self) {
        for i in &self.arr {
            println!("{:?}", i);
        }
    }
}


fn main() {
    let strings = vec!["Crypto", "Merkle", "Rust", "Tree"];
    let merkle = MerkleTree::new(&strings);
    merkle.print_arr();
}
