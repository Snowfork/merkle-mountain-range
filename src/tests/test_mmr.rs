use super::{MergeNumberHash, NumberHash};
use crate::{leaf_index_to_mmr_size, util::MemStore, Error, MMR};
use faster_hex::hex_string;
use proptest::prelude::*;
use rand::{seq::SliceRandom, thread_rng};
use bytes::Bytes;

fn test_mmr(count: u32, proof_elem: Vec<u32>) {
    let store = MemStore::default();
    let mut mmr = MMR::<_, MergeNumberHash, _>::new(0, &store);
    let positions: Vec<u64> = (0u32..count)
        .map(|i| mmr.push(NumberHash::from(i)).unwrap())
        .collect();
    let root = mmr.get_root().expect("get root");
    let proof = mmr
        .gen_proof(
            proof_elem
                .iter()
                .map(|elem| positions[*elem as usize])
                .collect(),
        )
        .expect("gen proof");
    mmr.commit().expect("commit changes");

    let result = proof
        .verify(
            root,
            proof_elem
                .iter()
                .map(|elem| (positions[*elem as usize], NumberHash::from(*elem)))
                .collect(),
        )
        .unwrap();
    assert!(result);
}

// fn test_gen_new_root_from_proof(count: u32) {
//     let store = MemStore::default();
//     let mut mmr = MMR::<_, MergeNumberHash, _>::new(0, &store);
//     let positions: Vec<u64> = (0u32..count)
//         .map(|i| mmr.push(NumberHash::from(i)).unwrap())
//         .collect();
//     let elem = count - 1;
//     let pos = positions[elem as usize];
//     let proof = mmr.gen_proof(vec![pos]).expect("gen proof");
//     let new_elem = count;
//     let new_pos = mmr.push(NumberHash::from(new_elem)).unwrap();
//     let root = mmr.get_root().expect("get root");
//     mmr.commit().expect("commit changes");
//     let calculated_root = proof
//         .calculate_root_with_new_leaf(
//             vec![(pos, NumberHash::from(elem))],
//             new_pos,
//             NumberHash::from(new_elem),
//             leaf_index_to_mmr_size(new_elem.into()),
//         )
//         .unwrap();
//     assert_eq!(calculated_root, root);
// }

// #[test]
// fn test_mmr_root() {
//     let store = MemStore::default();
//     let mut mmr = MMR::<_, MergeNumberHash, _>::new(0, &store);
//     (0u32..11).for_each(|i| {
//         mmr.push(NumberHash::from(i)).unwrap();
//     });
//     let root = mmr.get_root().expect("get root");
//     let hex_root = hex_string(&root.0).unwrap();
//     assert_eq!(
//         "f6794677f37a57df6a5ec36ce61036e43a36c1a009d05c81c9aa685dde1fd6e3",
//         hex_root
//     );
// }

// #[test]
// fn test_empty_mmr_root() {
//     let store = MemStore::<NumberHash>::default();
//     let mmr = MMR::<_, MergeNumberHash, _>::new(0, &store);
//     assert_eq!(Err(Error::GetRootOnEmpty), mmr.get_root());
// }

// #[test]
// fn test_mmr_3_peaks() {
//     test_mmr(11, vec![5]);
// }

// #[test]
// fn test_mmr_2_peaks() {
//     test_mmr(10, vec![5]);
// }

// #[test]
// fn test_mmr_1_peak() {
//     test_mmr(7, vec![5]);
// }

// #[test]
// fn test_mmr_first_elem_proof() {
//     test_mmr(11, vec![0]);
// }

#[test]
fn test_calculate_peak_root() {

    println!("----------- test_calculate_peak_root -----------");
    let store = MemStore::default();
    let mut mmr = MMR::<_, MergeNumberHash, _>::new(0, &store);

    // Push leaves 0-6 to the MMR as number hashes
    println!("Appending leaves [0-6]");
    let numLeaves = 7;
    let positions: Vec<u64> = (0u32..numLeaves)
    .map(|i| mmr.push(NumberHash::from(i)).unwrap())
    .collect();

    // Input integers 0-6 are hashed into the following leaves:
    // let leafHashes = [
    //     "0xc3e7ba6b511162fead58f2c8b5764ce869ed1118011ac37392522ed16720bbcd",
    //     "0x037ff5a3903a59630e03b84cda912c26bf19442efe2cd30c2a25547e06ded385",
    //     "0xffb0ad2811094c7f63826e33b6d7b3afa72587be856a86f10e3d0d869bbc37e5",
    //     "0xf47e991c124932a8573f782e1bc2fa62f628f8e1e074e6193170b0e302e37421",
    //     "0xd2ad83af7d5b0387eb704e57f4539138df402d72704fb74fde7a33353fab598d",
    //     "0xe232c7350837c9d87a948ddfc4286cc49d946e8cdad9121e91595f190ed7e54d",
    //     "0x32d44b4a8e8a3046b9c02315847eb091678a59f136226e70d66f3a82bd836ce1"
    // ];

    // Get the actual root and log
    let root = mmr.get_root().expect("get root");
    println!("root: {:?}", root);

    // We want the proof for index 5
    let proof_elem = [2, 5, 6];

    // Calculate leaf data (it's just [2, 5, 6])
    let leaf_data = proof_elem
        .iter()
        .map(|elem| positions[*elem as usize])
        .collect();

    // Generate the proof and commit it to the MMR
    let proof = mmr
        .gen_proof(
            leaf_data,
        )
        .expect("gen proof");
    mmr.commit().expect("commit changes");

    // Calculate the proof element hashes using NumberHash helper
    let leaf_hashes = proof_elem
        .iter()
        .map(|elem| (positions[*elem as usize], NumberHash::from(*elem)))
        .collect();

    // Calculate the root and log
    // 1. calculate_root calls
    // 1a. calculate_peaks_hashes -> returns bagged peaks [0123, 45, 6]
    // 1b. bagging_peaks_hashes -> [6, 45] -> [645, 0123] -> [0123456] = root
    let calculatedRoot = proof.calculate_root(leaf_hashes);
    println!();
    println!("calculatedRoot: {:?}", calculatedRoot);
}

// #[test]
// fn test_mmr_last_elem_proof() {
//     test_mmr(11, vec![10]);
// }

// #[test]
// fn test_mmr_1_elem() {
//     test_mmr(1, vec![0]);
// }

// #[test]
// fn test_mmr_2_elems() {
//     test_mmr(2, vec![0]);
//     test_mmr(2, vec![1]);
// }

// #[test]
// fn test_mmr_2_leaves_merkle_proof() {
//     test_mmr(11, vec![3, 7]);
//     test_mmr(11, vec![3, 4]);
// }

// #[test]
// fn test_mmr_2_sibling_leaves_merkle_proof() {
//     test_mmr(11, vec![4, 5]);
//     test_mmr(11, vec![5, 6]);
//     test_mmr(11, vec![6, 7]);
// }

// #[test]
// fn test_mmr_3_leaves_merkle_proof() {
//     // test_mmr(11, vec![4, 5, 6]);
//     // test_mmr(11, vec![3, 5, 7]);
//     // test_mmr(11, vec![3, 4, 5]);
//     // test_mmr(100, vec![3, 5, 13]);
// }

// #[test]
// fn test_gen_root_from_proof() {
//     test_gen_new_root_from_proof(11);
// }

// prop_compose! {
//     fn count_elem(count: u32)
//                 (elem in 0..count)
//                 -> (u32, u32) {
//                     (count, elem)
//     }
// }

// proptest! {
//     #[test]
//     fn test_random_mmr(count in 10u32..500u32) {
//         let mut leaves: Vec<u32> = (0..count).collect();
//         let mut rng = thread_rng();
//         leaves.shuffle(&mut rng);
//         let leaves_count = rng.gen_range(1, count - 1);
//         leaves.truncate(leaves_count as usize);
//         test_mmr(count, leaves);
//     }

//     #[test]
//     fn test_random_gen_root_with_new_leaf(count in 1u32..500u32) {
//         test_gen_new_root_from_proof(count);
//     }
// }
