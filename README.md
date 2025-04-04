# Merkle Tree
Implementation of a Merkle tree in Rust.

# Why?

The purpose of this project is to learn about the Merkle Tree, a widely used structure in distributed systems for data verification, while implementing it.

# What?

A Merkle Tree is a tree in which every leaf is a hash. Then, every node that is not a leaf is a hash of the concatenation of its child nodes.
From this structure we get efficient and secure verification of large pieces of data. The use of this data structure is really broad and 
can be used on the cryptography world or in Peer to Peer protocols like BitTorrent.

# How?

To run the main program you must run:

```sh
make
```

To run the tests you must run

```sh
make test
```

# Implementations

This implementation will support:
- The Merkle Tree can be built out of an array.
- The Merkle Tree can generate a proof that it contains an element.
- The Merkle Tree can verify that a given hash is contained in it.
- The Merkle Tree can be dynamic, this means that elements can be added once it is built.

# Uses

- Rust 1.85.0

# References

- Brilliant: https://brilliant.org/wiki/merkle-tree/
- Smart Contract Programmer: https://www.youtube.com/watch?v=n6nEPaE7KZ8 
- Decentralized Thoughts: https://decentralizedthoughts.github.io/2020-12-22-what-is-a-merkle-tree/
