#[cfg(test)]
mod test {
    use sp_core::Blake2Hasher;
    use sp_trie::{
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
}
