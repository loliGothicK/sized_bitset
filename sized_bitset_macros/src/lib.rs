use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn generate_num_traits(_: TokenStream) -> TokenStream {
    (1usize..=128usize)
        .map(|size| -> proc_macro2::TokenStream {
            let to_u8 = (size <= 8).then(|| {
                let index = syn::Index::from(size);
                quote! {
                    impl convert::To8 for SizedBitset<#index> {
                        fn to_u8(&self) -> u8 {
                            self.bits
                                .iter()
                                .enumerate()
                                .map(|(idx, bit)| (*bit as u8) << (idx as u8))
                                .fold(0, |mut sum, x| {
                                    sum += x;
                                    sum
                                })
                        }
                    }
                }
            });
            let to_u16 = (size <= 16).then(|| {
                let index = syn::Index::from(size);
                quote! {
                    impl convert::To16 for SizedBitset<#index> {
                        fn to_u16(&self) -> u16 {
                            self.bits
                                .iter()
                                .enumerate()
                                .map(|(idx, bit)| (*bit as u16) << (idx as u16))
                                .fold(0, |mut sum, x| {
                                    sum += x;
                                    sum
                                })
                        }
                    }
                }
            });
            let to_u32 = (size <= 32).then(|| {
                let index = syn::Index::from(size);
                quote! {
                    impl convert::To32 for SizedBitset<#index> {
                        fn to_u32(&self) -> u32 {
                            self.bits
                                .iter()
                                .enumerate()
                                .map(|(idx, bit)| (*bit as u32) << (idx as u32))
                                .fold(0, |mut sum, x| {
                                    sum += x;
                                    sum
                                })
                        }
                    }
                }
            });
            let to_u64 = (size <= 64).then(|| {
                let index = syn::Index::from(size);
                quote! {
                    impl convert::To64 for SizedBitset<#index> {
                        fn to_u64(&self) -> u64 {
                            self.bits
                                .iter()
                                .enumerate()
                                .map(|(idx, bit)| (*bit as u64) << (idx as u64))
                                .fold(0, |mut sum, x| {
                                    sum += x;
                                    sum
                                })
                        }
                    }
                }
            });
            let to_u128 = (size <= 128).then(|| {
                let index = syn::Index::from(size);
                quote! {
                    impl convert::To128 for SizedBitset<#index> {
                        fn to_u128(&self) -> u128 {
                            self.bits
                                .iter()
                                .enumerate()
                                .map(|(idx, bit)| (*bit as u128) << (idx as u128))
                                .fold(0, |mut sum, x| {
                                    sum += x;
                                    sum
                                })
                        }
                    }
                }
            });

            [
                quote! {#to_u8},
                quote! {#to_u16},
                quote! {#to_u32},
                quote! {#to_u64},
                quote! {#to_u128},
            ]
            .into_iter()
            .collect()
        })
        .collect::<proc_macro2::TokenStream>()
        .into()
}
