use merkle::MerkleTree;

mod merkle;



fn main() {
    let strings = vec!["Crypto", "Merkle", "Rust", "Tree", "Test", "Crypto", "Merkle", "Rust", "Tree", "Test"];
    let merkle = MerkleTree::new(strings);
    println!("{:?}", merkle.contains(3, 0));
}
