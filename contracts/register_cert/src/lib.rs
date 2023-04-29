#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod register_cert {

    #[ink(storage)]
    pub struct RegisterCert {
        value: bool,
    }

    impl RegisterCert {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn flip(&mut self) -> bool {
            self.value = !self.value;
            self.value
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let register_cert = RegisterCert::default();
            assert_eq!(register_cert.get(), false);
        }

        #[ink::test]
        fn it_works() {
            let mut register_cert = RegisterCert::new(false);
            assert_eq!(register_cert.get(), false);
            register_cert.flip();
            assert_eq!(register_cert.get(), true);
        }
    }
}
