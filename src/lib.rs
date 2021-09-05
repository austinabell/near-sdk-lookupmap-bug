
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
            let mut v = l.get(&aid).unwrap();
            v.iter().for_each(|x|{
                println!("{}",x);
            });
            // ! Can't use us to delete here because it is being read from v
            // us.remove(&uid);
            v.remove(&uid);
            v.iter().for_each(|x|{
                println!("{}",x);
            });
        }
    }

    mod near_sdk_3 {
        use near_sdk::json_types::ValidAccountId;
        use near_sdk::collections::{LookupMap, UnorderedSet};
        use near_sdk::{MockedBlockchain, env, testing_env};
        use std::convert::TryFrom;
        use near_sdk::test_utils::VMContextBuilder;

        #[test]
        fn empty_lookup_maps() {
            testing_env!(VMContextBuilder::new().build());
            let uid = 4u64;
            let aid = ValidAccountId::try_from("a.testnet").unwrap();
            let mut l:LookupMap<ValidAccountId,UnorderedSet<u64>> = LookupMap::new(b"e".to_vec());
            let mut us:UnorderedSet<u64> = UnorderedSet::new(b"dw".to_vec());
            us.insert(&uid);
            l.insert(&aid, &us);
            println!("Hello, world!");
            let mut v = l.get(&aid).unwrap();
            v.iter().for_each(|x|{
                println!("{}",x);
            });
            // ! Can't use us to delete here because it is being read from v
            // us.remove(&uid);
            v.remove(&uid);
            v.iter().for_each(|x|{
                println!("{}",x);
            });
        }
    }



}