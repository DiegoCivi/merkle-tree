use std::hash::{DefaultHasher, Hash, Hasher};

const BASE: i32 = 2;

type TreeStructure = Vec<Vec<u64>>;

/// Abstraction of a Merkle Tree. The structure is represented
/// as a vector of vectors. Each vector contains hashes and represents
/// a level in the tree. This structure is used so as to follow
/// the simple verification algorithm in this video:
/// https://www.youtube.com/watch?v=n6nEPaE7KZ8
pub struct MerkleTree {
    arr: TreeStructure,
}

impl MerkleTree {

    /// Creates a new MerkleTree
    /// 
    /// ### Arguments
    /// 
    /// - `elements`: A vector with the elements that will be hashed and form the first level in the tree.
    /// 
    /// ### Returns
    /// 
    /// A MerkleTree instance 
    pub fn new<T: Hash + Clone>(elements: Vec<T>) -> Self {
        // Hash every element of the array
        let hashed_elements = create_first_level(elements);
        let arr = create_remaining_levels(hashed_elements);
        Self { arr }
    }

    /// Checks if the hash received is equal to the root of the tree
    /// 
    /// ### Arguments
    /// 
    /// - `hash_to_check`: A hash that will be compared with the root
    /// 
    /// ### Returns
    /// 
    /// If the hash is equal to the one of the root, then it returns true,
    /// else false.
    fn is_root(&self, hash_to_check: u64) -> bool {
        self.arr.last().unwrap().last().unwrap();

        let root_level = match self.arr.last() {
            Some(root_level) => root_level,
            None => return false,
        };

        match root_level.last() {
            Some(root) => *root == hash_to_check,
            None => false,
        }
    }

    /// Checks if the root of the tree can be obtained with the use of a proof, 
    /// a leaf and its index on the input array.
    /// 
    /// ### Arguments
    /// 
    /// - `proofs`: A vector of hashes that make up the proof to get to the root.
    /// - `leaf_index`: The index in the input array of the received leaf.
    /// - `leaf`: The hash of one of the elements on the input array.
    /// 
    /// ### Returns
    /// 
    /// A bool that is true if the root can be obtained with that information, false otherwise
    pub fn verify(&self, proofs: Vec<u64>, leaf_index: usize, leaf: u64) -> bool {
        let mut hash_index = leaf_index;
        let mut hash = leaf;
        let mut concatenation: String;
        for proof in &proofs {

            if hash_index % 2 == 0 {
                // We know that if the index is even, the proof is on the right: hash + proof
                concatenation = concatenate_elements(hash, *proof);
            } else {
                // We know that if the index is odd, the proof is on the left: proof + hash
                concatenation = concatenate_elements(*proof, hash);
            }

            // Get the new hash and update the index for the next level 
            hash = hash_element(concatenation);
            hash_index /= 2;
        }

        self.is_root(hash)
    }

    pub fn generate_proof(&self, mut hash_index: usize) -> Vec<u64> {
        let mut proof_hash: u64;
        let mut proof = Vec::new();
        for level in &self.arr {
            // If we reach the root level we dont continue
            // since the root does not go on the proof.
            if level.len() == 1 {
                break;
            }

            if hash_index % 2 == 0 {
                proof_hash = level[hash_index + 1];
            } else {
                proof_hash = level[hash_index - 1];
            }
            proof.push(proof_hash);
            hash_index /= 2;
        }
        proof
    }

    pub fn add_element<T: Hash + Clone>(&mut self, new_elem: T) {
        let curr_base_len = self.arr[0].len();
        self.arr[0].push(hash_element(new_elem));
        extend_elements(&mut self.arr[0]);
        let new_base_section = self.arr[0][curr_base_len..].to_vec();
        let subtree = create_remaining_levels(new_base_section);
        
        for i in 1..self.arr.len() {
            self.arr[i].extend(subtree[i].clone());
        }

        // Create the new root
        let last_level = self.arr.last().unwrap(); // TODO: Remove the unwrap
        let concatenated_roots = concatenate_elements(last_level[0], last_level[1]);
        let new_root = hash_element(concatenated_roots);
        let new_root_level = vec![new_root];
        self.arr.push(new_root_level);
    }
}

/// Concatenates to elements into one
/// 
/// ### Arguments
/// 
/// - `elem1`: An u64 that will be the first part of the concatenation.
/// - `elem2`: An u64 that will be the second part of the concatenation.
/// 
/// ### Returns
/// 
/// A String thats the result of the concatenation fo the 2 elements
fn concatenate_elements(elem1: u64, elem2: u64) -> String {// TODO: Check if this way of concatenating the hashes is correct
    elem1.to_string() + &elem2.to_string()
}

/// Hashes an element
/// 
/// ### Arguments
/// 
/// - `element`: An element that implements the trait Hash
/// 
/// ### Returns
/// 
/// An u64 that represents the hash of the element
fn hash_element<T: Hash>(element: T) -> u64 {
    let mut hasher = DefaultHasher::new();
    element.hash(&mut hasher);
    hasher.finish()
}

fn diff_to_power_of_2(num: f64) -> i32 {
    // Find the exponent that would get us close to the len of the elements vector 
    let exp = num.log2().ceil() as u32;
    // Get how much more elements we need to get to a power of 2 len
    let diff = BASE.pow(exp) - num as i32;
    diff
}

/// Extends the elements vector so it has a len of
/// equal to a power of 2, if necessary
/// 
/// First we need to find the exponent that would give us
/// a close value to the elements len. Once we have this, we
/// can get the difference between the closes power of 2 and
/// the current len. That difference is the amount of repeated
/// cells we have to add again to make the len of to be a
/// power of 2.
/// 
/// ### Arguments
/// 
/// - `elements`: A vector with the elements that will be hashed and form the first level in the tree
fn extend_elements<T: Hash + Clone>(elements: &mut Vec<T>) { // TODO: Check if this function should be inside the impl
    let diff = diff_to_power_of_2(elements.len() as f64);
    if diff != 0 {
        // Add the last 'diff' elements to the elements vector
        let index = elements.len() - diff as usize;
        let elements_slice = elements[index..].to_vec();
        elements.extend(elements_slice);
    }
}

/// Creates the first level of the Merkle Tree.
/// 
/// Hashes all the input elements and adding repeated hashes 
/// if the len is not equal to a power of 2.
/// 
/// ### Arguments
/// 
/// - `elements`: A vector with the elements that will be hashed and form the first level in the tree
/// 
/// ### Returns
/// 
/// A vector full of the hashes of the elements. This vector represents the first
/// level of the Merkle Tree
fn create_first_level<T: Hash + Clone>(mut elements: Vec<T>) -> Vec<u64> { // TODO: Check if this function should be inside the impl
    extend_elements(&mut elements);
    elements.iter().map(|elem| {
        hash_element(elem)
    }).collect()
}

/// Uses the first level of the tree to create the remaining levels.
/// Each new level uses the one before.
/// 
/// ### Arguments
/// 
/// - `hashed_elements`: A vector full of hashes representing the first level of the tree
/// 
/// ### Returns
/// 
/// A vector of vectors with hashes. Each vector represents a level on the tree, 
/// starting from the first to the last (the root).
fn create_remaining_levels(hashed_elements: Vec<u64>) -> TreeStructure { // TODO: Check if this function should be inside the impl
    // We create the vec that will contain each level of the tree.
    // Then we add the first level (the already hashed elements we have).
    let mut vec = Vec::new();
    vec.push(hashed_elements.clone());

    // Each level creates the next level. So we iter each level by taking
    // chunks of size 2, concatenating this chunks and hashing the concatenation.
    // This process creates the next level.
    let mut hashes = hashed_elements;
    while hashes.len() != 1 {
        hashes = hashes.chunks(2).map(|chunk| {
            let concatenated = concatenate_elements(chunk[0], chunk[1]);
            hash_element(concatenated)
        }).collect();
        vec.push(hashes.clone());
    }
    vec
}


#[cfg(test)]
mod tests {
    use super::*;
    const LEVEL_0: usize = 0;
    const LEVEL_1: usize = 1;
    const LEVEL_2: usize = 2;


    /// Manually generates the tree structure while also generating a tree.
    /// All with the same input array, which has 4 elements in total.
    /// The structure should look like this:
    /// 
    ///             [
    /// LEVEL 0         [elem0_hash, elem1_hash, elem2_hash, elem3_haash],
    /// LEVEL 1         [elem01_hash, elem23_hash],
    /// LEVEL 2         [elem0123_hash],
    ///             ]
    /// From this we can see how the level 0 contains every hash of the elements
    /// while level 2 has the root.
    /// 
    fn manually_create_tree_hashes() -> (TreeStructure, MerkleTree) {
        let data = vec!["Crypto", "Merkle", "Rust", "Tree"];
        let mut tree = Vec::new();
        let merkle = MerkleTree::new(data.clone());
        // Get the hashes of the elements and manually create the tree structure
        // Level 0. It has the hashes of every element
        let elem0_hash = hash_element(data[0]);
        let elem1_hash = hash_element(data[1]);
        let elem2_hash = hash_element(data[2]);
        let elem3_hash = hash_element(data[3]);

        let level_0 = vec![elem0_hash, elem1_hash, elem2_hash, elem3_hash];

        // Level 1. It has the hashes of:
        // (elem0_hash + elem1_hash) = elem01_hash 
        // (elem2_hash + elem3_hash) = elem23_hash
        let elem01 = concatenate_elements(elem0_hash, elem1_hash);
        let elem01_hash = hash_element(elem01);

        let elem23 = concatenate_elements(elem2_hash, elem3_hash);
        let elem23_hash = hash_element(elem23);

        let level_1 = vec![elem01_hash, elem23_hash];

        // Level 2. It only contains one hash which will be the root:
        // (elem01_hash + elem23_hash) = root_hash
        let root = concatenate_elements(elem01_hash, elem23_hash);
        let root_hash = hash_element(root);

        let level_2 = vec![root_hash];

        // Push every level so we can have a manually created tree
        tree.push(level_0);
        tree.push(level_1);
        tree.push(level_2);

        (tree, merkle)
    }

    #[test]
    /// Test if the concatenation differs when changing order of elements
    fn hash_depends_on_concat_order() {
        // Declare our elements
        let elem1 = String::from("Crypto");
        let elem2 = String::from("Rust");
        // Hash our elements
        let hash_1  = hash_element(elem1);
        let hash_2  = hash_element(elem2);

        // Create the hash of the concatenation hash_1 + hash_2
        let concat_12 = concatenate_elements(hash_1, hash_2);
        let hash_12 = hash_element(concat_12);
        // Create the hash of the concatenation hash_2 + hash_1
        let concat_21 = concatenate_elements(hash_2, hash_1);
        let hash_21 = hash_element(concat_21);

        assert_ne!(hash_12, hash_21);
    }

    #[test]
    /// Test if by passing an input array of 3 items we get one of 4
    /// items with the last 2 being equal.
    fn extend_elements_repeats_last_one() {
        let mut data = vec!["Crypto", "Merkle", "Rust"];
        let expected_result = vec!["Crypto", "Merkle", "Rust", "Rust"];
        extend_elements(&mut data);

        assert_eq!(data, expected_result);
    }

    #[test]
    /// Test the case where the input array has only value
    /// 
    /// The creation of the Merkle Tree with an input array of only one value
    /// should just contain the hash of that value and nothing else.
    fn creation_from_arrray_one_value() {
        let data = vec!["Crypto"];
        let merkle = MerkleTree::new(data.clone());

        assert_eq!(merkle.arr.len(), 1);
    }

    #[test]
    /// Test the creation of a Merkle Tree
    /// 
    /// We check if the hashes are correct and also if the number of
    /// levels is the expected when creating a tree from an array of
    /// 4 initial elements.
    fn creation_from_arrray() {
        // We know that when we use an input array of 4 elements
        // the quantity of levels should be 3.
        let desired_level_quantity = 3;
       
        let (manual_tree, merkle) = manually_create_tree_hashes();

        // Test every level
        assert_eq!(merkle.arr[LEVEL_0], manual_tree[LEVEL_0]);
        assert_eq!(merkle.arr[LEVEL_1], manual_tree[LEVEL_1]);
        assert_eq!(merkle.arr[LEVEL_2], manual_tree[LEVEL_2]);
        // Test quantity of levels
        assert_eq!(merkle.arr.len(), desired_level_quantity);
    }

    #[test]
    /// Test the creation of a Merkle Tree with an input array that
    /// has a len that is not a power of 2.
    /// 
    /// With an input array of 5 elements, the Merkle Tree should
    /// copy repeated elements to have a first level with a
    /// quantity of 8 elements. Then with 8 elements the tree
    /// should have 4 different levels.
    fn creation_from_array_5_elements() {
        let desired_level_quantity = 4;
        let desired_quantity_in_first_level = 8;
        let data = vec!["Crypto", "Merkle", "Rust", "Tree", "Test"];
        let merkle = MerkleTree::new(data);

        assert_eq!(merkle.arr[LEVEL_0].len(), desired_quantity_in_first_level);
        assert_eq!(merkle.arr.len(), desired_level_quantity);
    }

    #[test]
    /// Test if the creation of a Merkle Tree with an input array that
    /// has a len that is not power of 2, has the correct hash values
    /// on the first level.
    /// 
    /// With 3 elements, the creation should copy the last element so
    /// the first level has 4 elements. The last and penultimate hashes
    /// in the first level should be the same.
    fn creation_from_array_3_elements() {
        let data = vec!["Crypto", "Merkle", "Rust"];
        let merkle = MerkleTree::new(data);
        let last_i = 3;
        let penultimate_i = 2;

        assert_eq!(merkle.arr[LEVEL_0][last_i], merkle.arr[LEVEL_0][penultimate_i]);
    }

    #[test]
    /// Test if the expected hash to be the root is actually the root 
    /// of the tree.
    fn is_root_returns_true() {
        let data = vec!["Crypto", "Merkle"];
        let merkle = MerkleTree::new(data.clone());

        // We manually get the root
        let elem0_hash = hash_element(data[0]);
        let elem1_hash = hash_element(data[1]);

        let root_concatenation = concatenate_elements(elem0_hash, elem1_hash);
        let desired_root = hash_element(root_concatenation);

        assert!(merkle.is_root(desired_root));
    }

    #[test]
    /// Test if a random hash is the root of the tree.
    fn is_root_returns_false() {
        let data = vec!["Crypto", "Merkle"];
        let merkle = MerkleTree::new(data.clone());

        // We manually get the root
        let elem0_hash = hash_element(data[0]);
        let elem1_hash = hash_element(data[1]);
        // We add garbage to the concatenation so the hash changes
        let garbage = "x";
        let root_concatenation = concatenate_elements(elem0_hash, elem1_hash) + garbage;
        let wrong_root = hash_element(root_concatenation);

        assert!(!merkle.is_root(wrong_root));
    }

    #[test]
    /// Test if the tree can verify a correct proof
    fn tree_verifies_proof() {
        let data = vec!["Crypto", "Merkle", "Rust", "Tree"];
        let merkle = MerkleTree::new(data.clone());

        // Get the hashes of the elements manually.
        // Level 0. It has the hashes of every element.
        let elem0_hash = hash_element(data[0]);
        let elem1_hash = hash_element(data[1]);
        let elem2_hash = hash_element(data[2]);
        let elem3_hash = hash_element(data[3]);

        // Create one of the proof hashes that we will be using:
        // (elem2_hash + elem3_hash) = elem23_hash
        let elem23 = concatenate_elements(elem2_hash, elem3_hash);
        let elem23_hash = hash_element(elem23);

        // Creation of the proof and the necessary index 
        let proof = vec![elem0_hash, elem23_hash];
        let elem1_index = 1;
         
        assert!(merkle.verify(proof, elem1_index, elem1_hash));
    }

    #[test]
    /// Test if the tree can verify an incorrect proof
    fn tree_cant_verify_wrong_proof() {
        let data = vec!["Crypto", "Merkle", "Rust", "Tree"];
        let merkle = MerkleTree::new(data.clone());

        // Get the hashes of the elements manually.
        // Level 0. It has the hashes of every element
        let elem0_hash = hash_element(data[0]);
        let elem1_hash = hash_element(data[1]);
        let elem2_hash = hash_element(data[2]);
        let elem3_hash = hash_element(data[3]);

        // Create one of the proofs that we will be using:
        // (elem2_hash + elem3_hash) = elem23_hash
        let garbage = "X";
        let elem23 = concatenate_elements(elem2_hash, elem3_hash) + garbage;
        let elem23_hash = hash_element(elem23);

        let proof = vec![elem0_hash, elem23_hash];
        let elem1_index = 1;
         
        assert!(!merkle.verify(proof, elem1_index, elem1_hash));
    }

    #[test]
    /// Test if passing the wrong index makes the varifying to fail
    fn verify_with_wrong_index() {
        let data = vec!["Crypto", "Merkle", "Rust", "Tree"];
        let merkle = MerkleTree::new(data.clone());

        // Get the hashes of the elements and manually create the tree structure
        // Level 0. It has the hashes of every element
        let elem0_hash = hash_element(data[0]);
        let elem1_hash = hash_element(data[1]);
        let elem2_hash = hash_element(data[2]);
        let elem3_hash = hash_element(data[3]);

        // Create one of the proofs that we will be using:
        // (elem2_hash + elem3_hash) = elem23_hash
        let garbage = "X";
        let elem23 = concatenate_elements(elem2_hash, elem3_hash) + garbage;
        let elem23_hash = hash_element(elem23);

        let proof = vec![elem0_hash, elem23_hash];
        let elem1_wrong_index = 2;
         
        assert!(!merkle.verify(proof, elem1_wrong_index, elem1_hash));
    }

    #[test]
    /// Test if the generation of proof works
    /// 
    /// By getting the manually created tree and the tree created with
    /// our abstraction, we manually create what would be the correct proof
    /// for the first element in the input array. We then check if the 
    /// generated proof is equal to the one we manually created.  
    fn generate_right_proof() {
        let (manual_tree, merkle) = manually_create_tree_hashes();

        let elem1_hash = manual_tree[LEVEL_0][1];
        let elem23_hash = manual_tree[LEVEL_1][1];

        let desired_proof = vec![elem1_hash, elem23_hash];
        let proof = merkle.generate_proof(0);

        assert_eq!(proof, desired_proof);
    }

    #[test]
    /// Test if adding a new element creates a new level on the tree
    /// 
    /// If we start a Merkle Tree with an input array of 4 elements,
    /// this will create a tree with 3 levels. If we add an element
    /// the base level grows, creating a new level on the tree.
    fn add_element_creates_new_level() {
        let data = vec!["Crypto", "Merkle", "Rust", "Tree"];
        let mut desired_merkle_levels = 3;
        let mut merkle = MerkleTree::new(data);

        assert_eq!(merkle.arr.len(), desired_merkle_levels);

        merkle.add_element("Test");
        desired_merkle_levels = 4;

        assert_eq!(merkle.arr.len(), desired_merkle_levels);
    }

    #[test]
    /// Test if adding a new element, doubles the quantity of
    /// base elements.
    /// 
    /// If we start a Merkle Tree with an input array of 4 elements,
    /// by adding an element we no longer have a base level with
    /// a quantity that is a power of 2. So to have that again
    /// we need to have repeated values. In this case, we should
    /// end up having a base level with 8 elements.
    fn add_element_doubles_base_elements() {
        let data = vec!["Crypto", "Merkle", "Rust", "Tree"];
        let mut desired_base_level_quantity = data.len();
        let mut merkle = MerkleTree::new(data);

        assert_eq!(merkle.arr[LEVEL_0].len(), desired_base_level_quantity);

        merkle.add_element("Test");
        desired_base_level_quantity *= 2;

        assert_eq!(merkle.arr[LEVEL_0].len(), desired_base_level_quantity);
    }

    #[test]
    /// Test if the base level elements are correct when adding a new element
    /// in a tree that already has a base level of 2^n different elements
    fn add_element_creates_correct_hashes() {
        let data = vec!["Crypto", "Merkle"];
        let new_elem = "Rust";
        let mut merkle = MerkleTree::new(data);
        let old_root = merkle.arr[1][0];

        merkle.add_element(new_elem);
        let new_elem_hash = hash_element(new_elem);

        assert_eq!(merkle.arr[LEVEL_0][2], new_elem_hash);
        assert_eq!(merkle.arr[LEVEL_0][3], new_elem_hash);
        assert!(!merkle.is_root(old_root));
    }
}
