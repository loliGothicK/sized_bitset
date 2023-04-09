#[cfg(test)]
mod test {
    use coverage_helper::test;
    use proptest::{prop_assert_eq, prop_assert_ne, proptest};
    use sized_bitset::{convert::*, SizedBitset};

    #[test]
    fn from_const() {
        const BITSET: SizedBitset<8> =
            SizedBitset::from_const([true, true, true, true, true, true, true, true]);
        for idx in 0..8 {
            assert!(BITSET[idx]);
        }
    }

    #[derive(Copy, Clone)]
    enum Switch {
        On,
        #[allow(unused)]
        Off,
    }

    impl From<Switch> for bool {
        fn from(value: Switch) -> Self {
            match value {
                Switch::On => true,
                Switch::Off => false,
            }
        }
    }

    #[test]
    fn from() {
        use Switch::*;
        let bitset: SizedBitset<8> = [On, On, On, On, On, On, On, On].into();
        for idx in 0..8 {
            assert!(bitset[idx]);
        }
    }

    #[test]
    fn default() {
        {
            let bitset: SizedBitset<8> = Default::default();
            for idx in 0..8 {
                assert!(!bitset[idx]);
            }
        }
        {
            let bitset = SizedBitset::<8>::new();
            for idx in 0..8 {
                assert!(!bitset[idx]);
            }
        }
    }

    proptest! {
        #[test]
        fn to_string_with(bits: u8) {
            let bitset: SizedBitset<8> = bits.into();
            let expected = format!("{bits:08b}").replace('1', "!").replace('0', "?");

            prop_assert_eq!(bitset.to_string_with('!', '?'), expected);
        }
    }

    proptest! {
        #[test]
        fn from_str(bits: u8) {
            let bitset: SizedBitset<8> = format!("{bits:08b}").as_str().parse().unwrap();

            prop_assert_eq!(bitset.to_u8(), bits);
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
        fn all(mut bitset: SizedBitset<4>) {
            if bitset.all() {
                prop_assert_eq!(bitset.to_u8(), 15);
            }
        }
    }

    proptest! {
        #[test]
        fn any(mut bitset: SizedBitset<4>) {
            if bitset.any() {
                prop_assert_ne!(bitset.to_u8(), 0);
            }
        }
    }

    proptest! {
        #[test]
        fn none(mut bitset: SizedBitset<4>) {
            if bitset.none() {
                prop_assert_eq!(bitset.to_u8(), 0);
            }
        }
    }

    proptest! {
        #[test]
        fn count(mut bitset: SizedBitset<8>) {
            prop_assert_eq!(bitset.count(), bitset.to_string().chars().filter(|b| b == &'1').count());
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
}
