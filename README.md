# validators-app-api

This is an unofficial API bindings for [validators.app](https://validators.app/)

## Usage

Add the crate into your `Cargo.toml`:

```toml
[dependencies]
validators-app-api = "0.1.0"
```

Then you can use it in your code:

```rust
use validators_app_api::prelude::*;
use std::collections::HashMap;

fn main() {
    let token = std::env::var("VALIDATORS_APP_TOKEN").unwrap();
    let client = Client::new(&token);
    
    // Get first 10 validators info sorted by name
    let validators = client.get_validators(Some(ValidatorsOrder::Name), Some(10)).unwrap();
    
    // Get blocks history for these validators
    let blocks_history = validators.iter().map(|validator| {
        // Get last 50 blocks stats for the validator
        let history = client.get_validator_block_history(&validator.account, Some(50)).unwrap();
        (validator.account.clone(), history)
    }).collect::<HashMap<_, _>>();
    
    println!("Top 10 validators history: {:?}", blocks_history);
}
```