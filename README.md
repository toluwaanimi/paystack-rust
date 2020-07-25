# Paystack

## Introduction
Paystack Package for Rust

## Usage

### Install Package
```bash

```

* Initialise Paystack
```rust
    pub async fn main() -> Result<(), Error> {
        let _paystack = Paystack::new("sk_test_00000000000000000000000000000".to_string());
        Ok(())
    }
...
```

>**NOTE**<br/>
>Ensure you pass your paystack secret key
* Initiate charge

```rust
...
pub async fn main() -> Result<(), Error> {
    let transaction = paystack.initialize("adebayo@emmanuel.com".to_string(), "5000".to_string()).await;
    println!("{:#?}", transaction);
    Ok(())
}
```

* Verify charge

```rust
...
pub async fn main() -> Result<(), Error> {
    let verify = paystack.verify_payment("xxxxxxxx".to_string()).await;
    println!("{:#?}", verify);
    Ok(())
}

```


* Charge charge

```rust
...
pub async fn main() -> Result<(), Error> {
    let charge_card = paystack.charge_card("AUTH_xxxxx".to_string(), "a@a.com".to_string(), "5000".to_string()).await;
    println!("{:#?}", charge_card);
    Ok(())
}
```

>**NOTE**<br/>
>Check the `example` directory to see a sample implementation
## Contribution

Please contribute 
