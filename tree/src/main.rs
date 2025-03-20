use merkle::MerkleTree;

mod merkle;



fn main() {
    let strings = vec!["Crypto", "Merkle", "Rust", "Tree", "Test", "Crypto", "Merkle", "Rust", "Tree", "Test"];
    let _merkle = MerkleTree::new(strings);
}
