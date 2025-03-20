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

    pub fn contains(&self, mut hash_to_check: u64, mut hash_index: usize) -> bool {
        let mut proof_index: usize;
        let mut proof: u64;
        let mut concatenation: String;
        for level in &self.arr {
            // If the len of the level is 1, it means we are at the root level.
            // If we are at the root level, on the varibale hash_index we should have
            // the root. So we don´t continue iterating.
            if level.len() == 1 {
                break;
            }

            if hash_index % 2 == 0 {
                // We know that if the index is even, the proof is on the right: hash + proof
                proof_index = hash_index + 1;
                proof = level[proof_index];
                concatenation = concatenate_elements(hash_to_check, proof);
            } else {
                // We know that if the index is odd, the proof is on the left: proof + hash
                proof_index = hash_index - 1;
                proof = level[proof_index];
                concatenation = concatenate_elements(proof, hash_to_check);
            }

            // Get the new hash and update the index for the next level 
            hash_to_check = hash_element(concatenation);
            hash_index /= 2;
        }

        self.is_root(hash_to_check)
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
    // Find the exponent that would get us close to the len of the elements vector 
    let exp = (elements.len() as f64).log2().ceil() as u32;
    // Get how much more elements we need to get to a power of 2 len
    let diff = BASE.pow(exp) - elements.len() as i32;
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
    const FIRST_LEVEL_I: usize = 0;

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
    /// levels is the expected.
    fn creation_from_arrray() {
        let data = vec!["Crypto", "Merkle", "Rust", "Tree"];
        let merkle = MerkleTree::new(data.clone());
        let desired_level_quantity = 3;

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

        // Level 3. It only contains one hash which will be the root:
        // (elem01_hash + elem23_hash) = root_hash
        let root = concatenate_elements(elem01_hash, elem23_hash);
        let root_hash = hash_element(root);

        let level_2 = vec![root_hash];

        // Test every level
        assert_eq!(merkle.arr[0], level_0);
        assert_eq!(merkle.arr[1], level_1);
        assert_eq!(merkle.arr[2], level_2);
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

        assert_eq!(merkle.arr[FIRST_LEVEL_I].len(), desired_quantity_in_first_level);
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

        assert_eq!(merkle.arr[FIRST_LEVEL_I][last_i], merkle.arr[FIRST_LEVEL_I][penultimate_i]);
    }
}
