use merkle::MerkleTree;
mod merkle;



fn main() {
    let strings = vec!["Crypto", "Merkle", "Rust", "Tree"];
    let merkle = MerkleTree::new(&strings);
    merkle.print_arr();
}
