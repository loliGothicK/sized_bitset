#[cfg(test)]
mod test {
    use coverage_helper::test;
    use proptest::{prop_assert_eq, proptest};
    use sized_bitset::SizedBitset;

    #[test]
    fn from_array() {
        const BITSET: SizedBitset<8> =
            SizedBitset::from_const([true, true, true, true, true, true, true, true]);
        for idx in 0..8 {
            assert!(BITSET[idx]);
        }
    }

    #[test]
    fn to_u8() {
        use sized_bitset::convert::To8;
        {
            let bitset = SizedBitset::from_const([true, true, true, true, true, true, true, true]);
            assert_eq!(bitset.to_u8(), u8::MAX);
        }
        {
            let bitset =
                SizedBitset::from_const([false, false, false, false, true, true, true, true]);
            assert_eq!(bitset.to_u8(), u8::MAX - 15);
        }
    }

    proptest! {
        #[test]
        fn display(bits: u8) {
            let bitset: SizedBitset<8> = bits.into();

            prop_assert_eq!(bitset.to_string(), format!("{bits:08b}"));
        }
    }

    proptest! {
        #[test]
        fn flip(mut bitset: SizedBitset<4>) {
            let original = bitset;
            bitset.flip();
            for i in 0..4 {
                prop_assert_eq!(original[i], !bitset[i]);
            }
        }
    }

    proptest! {
        #[test]
        fn flipped(bitset: SizedBitset<4>) {
            let original = bitset;
            for i in 0..4 {
                prop_assert_eq!(original[i], !bitset.flipped()[i]);
            }
        }
    }
}
