use merkle::{calculate_elements_hashes, MerkleTree};
mod merkle;



fn main() {
    let strings = vec!["Crypto", "Merkle", "Rust", "Tree", "Test", "Crypto", "Merkle", "Rust", "Tree", "Test"];
    // let merkle = MerkleTree::new(&strings);
    calculate_elements_hashes(&strings);
}
