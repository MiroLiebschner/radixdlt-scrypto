use crate::utils::*;
use scrypto::constructs::*;
use scrypto::resource::*;
use scrypto::types::*;
use scrypto::*;

blueprint! {
    struct ResourceTest {
        resource: Address,
        tokens: Tokens,
        secret: String,
    }

    impl ResourceTest {
        pub fn create_mutable() -> Tokens {
           let resource = create_mutable_tokens("r1", Context::package_address());
           mint_tokens(resource, 100)
        }

        pub fn create_immutable() -> Tokens {
           create_immutable_tokens("r2", 100.into())
        }

        pub fn query() -> String {
            let resource: Resource = create_mutable_tokens("r3", Context::package_address()).into();
            resource.get_info().url
        }
    }
}