#[cfg(test)]
mod test {
    use sp_core::Blake2Hasher;
    use sp_trie::{
        trie_types::TrieDBMutBuilderV1, LayoutV0, LayoutV1, MemoryDB, TrieDBMut, TrieDBMutBuilder,
        TrieHash, TrieMut,
    };

    #[test]
    fn root_from_entries() {
        let mut db: MemoryDB<Blake2Hasher> = MemoryDB::new(&[0u8]);
        let mut root: TrieHash<LayoutV1<Blake2Hasher>> = Default::default();
        let mut t: TrieDBMut<'_, LayoutV1<Blake2Hasher>> =
            TrieDBMutBuilder::new(&mut db, &mut root).build();

        let entries: Vec<(&[u8], &[u8])> = vec![
            // "alfa" is at a hash-referenced leaf node.
            (b"alfa", &[0; 40]),
            // "bravo" is at an inline leaf node.
            (b"bravo", b"bravo"),
            // "do" is at a hash-referenced branch node.
            (b"do", b"verb"),
            // "dog" is at a hash-referenced branch node.
            (b"dog", &[0; 40]),
            // "doge" is at a hash-referenced leaf node.
            (b"doge", &[0; 40]),
            // extension node "o" (plus nibble) to next branch.
            (b"horse", b"stallion"),
            (b"house", b"building"),
        ];

        for entry in entries {
            let _ = t.insert(entry.0, entry.1).expect("inserted");
        }

        println!("ROOT {:?}", t.root());
    }

    #[test]
    fn root_from_entries_2() {
        let mut db1: MemoryDB<Blake2Hasher> = MemoryDB::new(&[0u8]);
        let mut db2: MemoryDB<Blake2Hasher> = MemoryDB::new(&[0u8]);

        {
            let mut rootv0: TrieHash<LayoutV0<Blake2Hasher>> = Default::default();
            let mut tv0: TrieDBMut<'_, LayoutV0<Blake2Hasher>> =
                TrieDBMutBuilder::new(&mut db1, &mut rootv0).build();

            let mut rootv1: TrieHash<LayoutV1<Blake2Hasher>> = Default::default();
            let mut tv1: TrieDBMut<'_, LayoutV1<Blake2Hasher>> =
                TrieDBMutBuilder::new(&mut db2, &mut rootv1).build();

            let entries: Vec<(&[u8], &[u8])> = vec![
                // "alfa" is at a hash-referenced leaf node.
                (b"alfa", &[0; 40]),
            ];

            for entry in entries {
                tv0.insert(entry.0, entry.1).expect("inserted");
                tv1.insert(entry.0, entry.1).expect("inserted");
            }

            println!("ROOT V0 {:?}", tv0.root());
            println!("ROOT V1 {:?}", tv1.root());
        }

        println!("key V0 {:?}", db1.keys());
        println!("key V1 {:?}", db2.keys());
    }

    mod from_external_file {
        use super::TrieMut;
        use parity_scale_codec::Decode;
        use std::{fs::File, io::BufReader};

        #[derive(serde::Deserialize, Debug)]
        pub struct Entries(Vec<String>);

        #[derive(parity_scale_codec::Decode)]
        pub struct Entry {
            pub key: Vec<u8>,
            pub value: Vec<u8>,
        }

        #[test]
        fn westend_14576856() {
            // IT COULD TAKE TIME (2 mins aprox)
            let filename = String::from("./src/14576856block_entries.json");

            let file = File::open(filename).unwrap();

            // json unmarshal
            let entries: Entries = serde_json::from_reader(BufReader::new(file)).unwrap();

            // scale decode
            let decoded_entries = entries
                .0
                .iter()
                .map(|string_encodec_hex| {
                    hex::decode(string_encodec_hex.replace("0x", "")).unwrap()
                })
                .map(|encoded_bytes| {
                    let encoded = encoded_bytes.clone();
                    let mut b = encoded.as_slice();
                    Entry::decode(&mut b).unwrap()
                });

            let entries = decoded_entries.map(|e| (e.key, e.value));

            let mut db = super::MemoryDB::new(&[0u8]);
            let mut root: super::TrieHash<super::LayoutV1<super::Blake2Hasher>> =
                Default::default();

            let mut t: super::TrieDBMut<'_, super::LayoutV1<super::Blake2Hasher>> =
                super::TrieDBMutBuilderV1::new(&mut db, &mut root).build();

            for entry in entries {
                let _ = t.insert(&entry.0, &entry.1).expect("inserted");
            }

            println!("{:?}", t.root());
        }
    }
}
