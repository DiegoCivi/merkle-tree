use merkle::MerkleTree;

mod merkle;



fn main() {
    let strings = vec!["Crypto", "Merkle", "Rust"];
    let mut merkle = MerkleTree::new(strings);
    merkle.add_element("Test");
    let proof = merkle.generate_proof(0).unwrap();
    let elem0_hash = 18444331223197392467;
    let verification = merkle.verify(proof, 0, elem0_hash);
    println!("Verification was succesful: {:?}", verification);
}
