use std::hash::{DefaultHasher, Hash, Hasher};

type TreeStructure = Vec<Vec<u64>>;

pub struct MerkleTree {
    arr: TreeStructure,
}

impl MerkleTree {

    pub fn new<T: Hash>(elements: &Vec<T>) -> Self {
        // TODO: Add support for vectors that have a len that is not a power of 2.
        // Hash every element of the array
        let hashed_elements = calculate_elements_hashes(elements);
        let arr = add_remaining_hashes(hashed_elements);
        Self { arr }
    }
}

// TODO: Check if this way of concatenating the hashes is correct
fn concatenate_elements(elem1: u64, elem2: u64) -> String {
    elem1.to_string() + &elem2.to_string()
}

fn hash_element<T: Hash>(element: T) -> u64 {
    let mut hasher = DefaultHasher::new();
    element.hash(&mut hasher);
    hasher.finish()
}


// TODO: Check if this function should be inside the impl
fn calculate_elements_hashes<T: Hash>(elements: &Vec<T>) -> Vec<u64> {
    elements.iter().map(|elem| {
        hash_element(elem)
    }).collect()
}

// TODO: Check if this function should be inside the impl
fn add_remaining_hashes(hashed_elements: Vec<u64>) -> Vec<Vec<u64>> {
    let mut arr = Vec::new();
    arr.push(hashed_elements.clone());
    let mut hashes = hashed_elements;
    while hashes.len() != 1 {
        
        hashes = hashes.chunks(2).map(|chunk| {
            let concatenated = concatenate_elements(chunk[0], chunk[1]);
            let temp = hash_element(concatenated);
            println!("({:?} + {:?}) = {:?}", chunk[0], chunk[1], temp);
            println!("****************");
            temp
        }).collect();
        arr.push(hashes.clone());
    }

    arr
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
    fn creation_from_arrray() {
        let data = vec!["Crypto", "Merkle", "Rust", "Tree"];
        let merkle = MerkleTree::new(&data);

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
    }
}
