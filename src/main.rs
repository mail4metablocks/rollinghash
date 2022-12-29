use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::clone::Clone;

fn rolling_hash<T: Hash + Eq>(data: &[T], chunk_size: usize) -> Vec<u64> {
    let mut hashes = Vec::new();

    // Split the data into chunks
    let chunks = data.chunks(chunk_size);

    // Calculate the hash of each chunk
    for chunk in chunks {
        let mut hasher = DefaultHasher::new();
        for item in chunk {
            item.hash(&mut hasher);
        }
        hashes.push(hasher.finish());
    }

    hashes
}

fn diff<T: Hash + Eq + Clone>(original: &[T], updated: &[T], chunk_size: usize) -> (Vec<usize>, Vec<T>) {
    let original_hashes = rolling_hash(original, chunk_size);
    let updated_hashes = rolling_hash(updated, chunk_size);

    let mut reused_chunks = Vec::new();
    let mut synchronized_chunks = Vec::new();

    // Compare the hashes of the original and updated data
    for (i, hash) in updated_hashes.iter().enumerate() {
        if *hash == original_hashes[i] {
            // Chunk can be reused from original data
            reused_chunks.push(i);
        } else {
            // Chunk has been modified or added and needs to be synchronized
            synchronized_chunks.extend_from_slice(&updated[i * chunk_size..(i + 1) * chunk_size]);
        }
    }

    (reused_chunks, synchronized_chunks)
}

#[test]
fn test_diff() {
    let original = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let updated = [1, 2, 3, 4, 5, 6, 8, 9, 10];

    let (reused_chunks, synchronized_chunks) = diff(&original, &updated, 3);

    assert_eq!(reused_chunks, [0, 1, 2]);
    assert_eq!(synchronized_chunks, [8, 9, 10]);
}

fn main() {
    let original = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let updated = [1, 2, 3, 4, 5, 6, 8, 9, 10];

    let (reused_chunks, synchronized_chunks) = diff(&original, &updated, 3);

    println!("Reused chunks: {:?}", reused_chunks);
    println!("Synchronized chunks: {:?}", synchronized_chunks);
}
