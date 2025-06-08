use ethers::types::Address;
use std::str::FromStr;
use ethers::core::k256::elliptic_curve::weierstrass::add;
// We want to use generics to basically filter by the trait we want to implement.

trait EthereumAddress {
    fn convert_address(&self) -> Result<Address, &'static str>;
}

impl EthereumAddress for &str {
    fn convert_address(&self) -> Result<Address, &'static str> {
        match Address::from_str(self) { 
            Ok(address) => Ok(address),
            Err(_) => Err("Invalid Ethereum Address String")
        }
    }
}

impl EthereumAddress for Address {
    fn convert_address(&self) -> Result<Address, &'static str> {
        Ok(*self)
    }
}

// We can define that T must be of a specific type
fn get_ethereum_data<T: EthereumAddress>(address: T) -> Address {
    let converted_address: Address = address.convert_address().unwrap();
    converted_address
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_polymorphism() {
        let addr: Address = Address::from_str("0x6d2e03b7EfFEae98BD302A9F836D0d6Ab0002766").unwrap();
        
        let new_addr: Address = get_ethereum_data(addr);
        assert_eq!(new_addr, Address::from_str("0x6d2e03b7EfFEae98BD302A9F836D0d6Ab0002766").unwrap());

        let new_addr: Address = get_ethereum_data("0x6d2e03b7EfFEae98BD302A9F836D0d6Ab0002766");
        assert_eq!(new_addr, Address::from_str("0x6d2e03b7EfFEae98BD302A9F836D0d6Ab0002766").unwrap());
    }

}