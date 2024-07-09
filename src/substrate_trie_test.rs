#[cfg(test)]
mod test {
    use sp_core::Blake2Hasher;
    use sp_trie::{
        generate_trie_proof,
        trie_types::{TrieDBBuilder, TrieDBMutBuilderV1},
        LayoutV1, MemoryDB, Recorder, Trie, TrieMut,
    };

    #[test]
    fn recorder() {
        let mut db = MemoryDB::<Blake2Hasher>::default();
        let mut root = Default::default();
        {
            let mut x = TrieDBMutBuilderV1::new(&mut db, &mut root).build();

            x.insert(b"pol", b"polvalue").unwrap();
            x.insert(b"polka", b"polkavalue").unwrap();
            x.insert(b"polkadot", b"polkadotvalue").unwrap();
            x.insert(b"go", b"govalue").unwrap();
            x.insert(b"gossamer", b"gossamervalue").unwrap();
        }

        {
            let mut recorder = Recorder::<LayoutV1<Blake2Hasher>>::new();
            let trie = TrieDBBuilder::new(&db, &root)
                .with_recorder(&mut recorder)
                .build();

            trie.get(b"go").unwrap().unwrap();

            let nodes: Vec<_> = recorder.drain().into_iter().map(|r| r.data).collect();
            assert_eq!(
                nodes,
                vec![
                    vec![
                        128, 192, 0, 128, 124, 255, 5, 248, 100, 180, 218, 180, 146, 187, 118, 79,
                        161, 92, 153, 38, 78, 48, 120, 69, 157, 112, 164, 176, 129, 164, 167, 36,
                        76, 131, 68, 6, 128, 42, 2, 217, 41, 157, 5, 134, 74, 180, 2, 124, 111,
                        183, 89, 195, 14, 111, 92, 59, 236, 175, 34, 115, 200, 121, 201, 142, 57,
                        123, 84, 26, 222
                    ],
                    vec![
                        195, 7, 111, 128, 0, 28, 103, 111, 118, 97, 108, 117, 101, 84, 75, 3, 115,
                        97, 109, 101, 114, 52, 103, 111, 115, 115, 97, 109, 101, 114, 118, 97, 108,
                        117, 101
                    ]
                ]
            );
        }
    }

    #[test]
    fn merkle_proof() {
        let mut db = MemoryDB::<Blake2Hasher>::default();
        let mut root = Default::default();
        {
            let mut x = TrieDBMutBuilderV1::new(&mut db, &mut root).build();

            x.insert(b"pol", b"pol").unwrap();
            x.insert(b"polka", b"polka").unwrap();
            x.insert(b"polkadot", b"polkadot").unwrap();
            x.insert(b"go", b"go").unwrap();
            x.insert(b"golang", b"golang").unwrap();
            x.insert(b"gossamer", b"gossamer").unwrap();
        }

        {
            let keys: Vec<&'static [u8]> = vec![b"go", b"polkadot"];
            let proof = generate_trie_proof::<LayoutV1<Blake2Hasher>, _, _, _>(&db, root, keys.iter())
                .expect("Proof generate failed");
            assert_eq!(
                proof,
                vec![
                    vec![
                        128, 192, 0, 0, 0
                    ], 
                    vec![
                        131, 7, 111, 192, 0, 48, 71, 12, 97, 110, 103, 24, 103, 111, 108, 97, 110, 103, 64, 75, 3, 115, 97, 109, 101, 114, 32, 103, 111, 115, 115, 97, 109, 101, 114
                    ], 
                    vec![
                        197, 0, 111, 108, 64, 0, 12, 112, 111, 108, 68, 195, 11, 97, 64, 0, 20, 112, 111, 108, 107, 97, 20, 69, 4, 111, 116, 0
                    ]
                ],
            );
        }
    }
}
