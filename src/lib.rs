
#[cfg(test)]
mod tests {

    mod near_sdk_4 {
        use near_sdk_4::AccountId;
        use near_sdk_4::collections::LookupMap;
        use near_sdk_4::collections::UnorderedSet;

        #[test]
        fn empty_lookup_map() {
            let uid = 4u64;
            let aid = AccountId::new_unchecked("a.testnet".to_lowercase());
            let mut l:LookupMap<AccountId,UnorderedSet<u64>> = LookupMap::new(b"e".to_vec());
            let mut us:UnorderedSet<u64> = UnorderedSet::new(b"dw".to_vec());
            us.insert(&uid);
            l.insert(&aid, &us);
            println!("Hello, world!");
            let v = l.get(&aid).unwrap();
            v.iter().for_each(|x|{
                println!("{}",x);
            });
            us.remove(&uid);
            v.iter().for_each(|x|{
                println!("{}",x);
            });
        }
    }

    mod near_sdk_3 {
        use near_sdk_3::json_types::ValidAccountId;
        use near_sdk_3::collections::{LookupMap, UnorderedSet};
        use near_sdk_3::{MockedBlockchain, env};
        use std::convert::TryFrom;

        #[test]
        fn empty_lookup_maps() {
            // let blockchain_interface =
            //     env::take_blockchain_interface().expect("Blockchain interface is not set");
            // let logs = blockchain_interface
            //     .as_mocked_blockchain()
            //     .expect("MockedBlockchain interface expected")
            //     .logs();
            // env::set_blockchain_interface(blockchain_interface);
            // let uid = 4u64;
            // let aid = ValidAccountId::try_from("few").unwrap();
            // let mut l:LookupMap<ValidAccountId,UnorderedSet<u64>> = LookupMap::new(b"e".to_vec());
            // let mut us:UnorderedSet<u64> = UnorderedSet::new(b"dw".to_vec());
            // us.insert(&uid);
            // l.insert(&aid, &us);
            // println!("Hello, world!");
            // let v = l.get(&aid).unwrap();
            // v.iter().for_each(|x|{
            //     println!("{}",x);
            // });
            // us.remove(&uid);
            // v.iter().for_each(|x|{
            //     println!("{}",x);
            // });
        }
    }



}