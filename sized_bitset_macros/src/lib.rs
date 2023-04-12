use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn generate_num_traits(_: TokenStream) -> TokenStream {
    (1usize..=128usize)
        .map(|size| -> proc_macro2::TokenStream {
            let index = syn::Index::from(size);

            let const_from_u8 = {
                let initializers = (0..size).map(|i| {
                    let idx = syn::Index::from(i);
                    if i < 8 {
                        quote! {
                            ((bits >> #idx) % 2) == 1
                        }
                    } else {
                        quote! {
                            false
                        }
                    }
                });
                (size >= 8).then(|| {
                    quote! {
                        impl SizedBitset<#index> {
                            pub const fn from_u8(bits: u8) -> Self {
                                Self::from_const([ #(#initializers),* ])
                            }
                        }
                    }
                })
            };

            let const_from_u16 = {
                let initializers = (0..size).map(|i| {
                    let idx = syn::Index::from(i);
                    if i < 16 {
                        quote! {
                            ((bits >> #idx) % 2) == 1
                        }
                    } else {
                        quote! {
                            false
                        }
                    }
                });
                (size >= 16).then(|| {
                    quote! {
                        impl SizedBitset<#index> {
                            pub const fn from_u16(bits: u16) -> Self {
                                Self::from_const([ #(#initializers),* ])
                            }
                        }
                    }
                })
            };

            let const_from_u32 = {
                let initializers = (0..size).map(|i| {
                    let idx = syn::Index::from(i);
                    if i < 32 {
                        quote! {
                            ((bits >> #idx) % 2) == 1
                        }
                    } else {
                        quote! {
                            false
                        }
                    }
                });
                (size >= 32).then(|| {
                    quote! {
                        impl SizedBitset<#index> {
                            pub const fn from_u32(bits: u32) -> Self {
                                Self::from_const([ #(#initializers),* ])
                            }
                        }
                    }
                })
            };

            let const_from_u64 = {
                let initializers = (0..size).map(|i| {
                    let idx = syn::Index::from(i);
                    if i < 64 {
                        quote! {
                            ((bits >> #idx) % 2) == 1
                        }
                    } else {
                        quote! {
                            false
                        }
                    }
                });
                (size >= 64).then(|| {
                    quote! {
                        impl SizedBitset<#index> {
                            pub const fn from_u64(bits: u64) -> Self {
                                Self::from_const([ #(#initializers),* ])
                            }
                        }
                    }
                })
            };

            let const_from_u128 = {
                let initializers = (0..size).map(|i| {
                    let idx = syn::Index::from(i);
                    if i < 128 {
                        quote! {
                            ((bits >> #idx) % 2) == 1
                        }
                    } else {
                        quote! {
                            false
                        }
                    }
                });
                (size >= 128).then(|| {
                    quote! {
                        impl SizedBitset<#index> {
                            pub const fn from_u128(bits: u128) -> Self {
                                Self::from_const([ #(#initializers),* ])
                            }
                        }
                    }
                })
            };

            let from_u8 = (size >= 8).then(|| {
                quote! {
                    impl From<u8> for SizedBitset<#index> {
                        fn from(bits: u8) -> Self {
                            unsafe {
                                (0..#index)
                                    .map(|i| if 8 > i { ((bits >> i) % 2) == 1 } else { false })
                                    .into_iter()
                                    .collect_vec()
                                    .as_slice()
                                    .try_into()
                                    .unwrap_unchecked()
                            }
                        }
                    }
                }
            });
            let from_u16 = (size >= 16).then(|| {
                let index = syn::Index::from(size);
                quote! {
                    impl From<u16> for SizedBitset<#index> {
                        fn from(bits: u16) -> Self {
                            unsafe {
                                (0..#index)
                                    .map(|i| if 16 > i { ((bits >> i) % 2) == 1 } else { false })
                                    .into_iter()
                                    .collect_vec()
                                    .as_slice()
                                    .try_into()
                                    .unwrap_unchecked()
                            }
                        }
                    }
                }
            });
            let from_u32 = (size >= 32).then(|| {
                let index = syn::Index::from(size);
                quote! {
                    impl From<u32> for SizedBitset<#index> {
                        fn from(bits: u32) -> Self {
                            unsafe {
                                (0..#index)
                                    .map(|i| if 32 > i { ((bits >> i) % 2) == 1 } else { false })
                                    .into_iter()
                                    .collect_vec()
                                    .as_slice()
                                    .try_into()
                                    .unwrap_unchecked()
                            }
                        }
                    }
                }
            });
            let from_u64 = (size >= 64).then(|| {
                let index = syn::Index::from(size);
                quote! {
                    impl From<u64> for SizedBitset<#index> {
                        fn from(bits: u64) -> Self {
                            unsafe {
                                (0..#index)
                                    .map(|i| if 64 > i { ((bits >> i) % 2) == 1 } else { false })
                                    .into_iter()
                                    .collect_vec()
                                    .as_slice()
                                    .try_into()
                                    .unwrap_unchecked()
                            }
                        }
                    }
                }
            });
            let from_u128 = (size >= 128).then(|| {
                let index = syn::Index::from(size);
                quote! {
                    impl From<u128> for SizedBitset<#index> {
                        fn from(bits: u128) -> Self {
                            unsafe {
                                (0..#index)
                                    .map(|i| if 128 > i { ((bits >> i) % 2) == 1 } else { false })
                                    .into_iter()
                                    .collect_vec()
                                    .as_slice()
                                    .try_into()
                                    .unwrap_unchecked()
                            }
                        }
                    }
                }
            });
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
                quote! {#from_u8},
                quote! {#from_u16},
                quote! {#from_u32},
                quote! {#from_u64},
                quote! {#from_u128},
                quote! {#const_from_u8},
                quote! {#const_from_u16},
                quote! {#const_from_u32},
                quote! {#const_from_u64},
                quote! {#const_from_u128},
            ]
            .into_iter()
            .collect()
        })
        .collect::<proc_macro2::TokenStream>()
        .into()
}
