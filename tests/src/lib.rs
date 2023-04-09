#[cfg(test)]
mod test {
    use std::ops::{BitAnd, BitOr, BitXor, Shl, ShlAssign, Shr, ShrAssign};

    use coverage_helper::test;
    use proptest::{prop_assert, prop_assert_eq, prop_assert_ne, proptest};
    use sized_bitset::{convert::*, SizedBitset};

    #[test]
    fn from_const() {
        const BITSET: SizedBitset<8> =
            SizedBitset::from_const([true, true, true, true, true, true, true, true]);
        for idx in 0..8 {
            assert!(BITSET[idx]);
        }
        let _ = SizedBitset::from_const([true, true, true, true, true, true, true, true]);
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

    #[test]
    #[should_panic]
    fn from_str_error() {
        let _bitset: SizedBitset<8> = "!?!?".parse().unwrap();
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

    proptest! {
        #[test]
        fn set(mut bitset: SizedBitset<4>) {
            for i in 0..4 {
                bitset.set(i);
                prop_assert!(bitset[i]);
            }
        }
    }

    proptest! {
        #[test]
        fn set_all(mut bitset: SizedBitset<4>) {
            bitset.set_all();
            for i in 0..4 {
                prop_assert!(bitset[i]);
            }
        }
    }

    proptest! {
        #[test]
        fn reset(mut bitset: SizedBitset<4>) {
            for i in 0..4 {
                bitset.reset(i);
                prop_assert!(!bitset[i]);
            }
        }
    }

    proptest! {
        #[test]
        fn reset_all(mut bitset: SizedBitset<4>) {
            bitset.reset_all();
            for i in 0..4 {
                prop_assert!(!bitset[i]);
            }
        }
    }

    proptest! {
        #[test]
        fn rotl(bits: u8) {
            let bitset: SizedBitset<8> = bits.into();
            for i in 0..=8 {
                prop_assert_eq!(bitset.rotl(i).to_u8(), bits.rotate_left(i as u32))
            }
        }
    }

    proptest! {
        #[test]
        fn rotr(bits: u8) {
            let bitset: SizedBitset<8> = bits.into();
            for i in 0..=8 {
                prop_assert_eq!(bitset.rotr(i).to_u8(), bits.rotate_right(i as u32))
            }
        }
    }

    proptest! {
        #[test]
        fn bitand(lhs: u8, rhs: u8) {
            let lhs_bitset: SizedBitset<8> = lhs.into();
            let rhs_bitset: SizedBitset<8> = rhs.into();
            prop_assert_eq!(lhs_bitset.bitand(rhs_bitset).to_u8(), lhs.bitand(rhs));
        }
    }

    proptest! {
        #[test]
        fn bitand_assign(lhs: u8, rhs: u8) {
            let mut lhs_bitset: SizedBitset<8> = lhs.into();
            let rhs_bitset: SizedBitset<8> = rhs.into();
            lhs_bitset &= rhs_bitset;
            prop_assert_eq!(lhs_bitset.to_u8(), lhs.bitand(rhs));
        }
    }

    proptest! {
        #[test]
        fn bitor(lhs: u8, rhs: u8) {
            let lhs_bitset: SizedBitset<8> = lhs.into();
            let rhs_bitset: SizedBitset<8> = rhs.into();
            prop_assert_eq!(lhs_bitset.bitor(rhs_bitset).to_u8(), lhs.bitor(rhs));
        }
    }

    proptest! {
        #[test]
        fn bitor_assign(lhs: u8, rhs: u8) {
            let mut lhs_bitset: SizedBitset<8> = lhs.into();
            let rhs_bitset: SizedBitset<8> = rhs.into();
            lhs_bitset |= rhs_bitset;
            prop_assert_eq!(lhs_bitset.to_u8(), lhs.bitor(rhs));
        }
    }

    proptest! {
        #[test]
        fn bitxor(lhs: u8, rhs: u8) {
            let lhs_bitset: SizedBitset<8> = lhs.into();
            let rhs_bitset: SizedBitset<8> = rhs.into();
            prop_assert_eq!(lhs_bitset.bitxor(rhs_bitset).to_u8(), lhs.bitxor(rhs));
        }
    }

    proptest! {
        #[test]
        fn bitxor_assign(lhs: u8, rhs: u8) {
            let mut lhs_bitset: SizedBitset<8> = lhs.into();
            let rhs_bitset: SizedBitset<8> = rhs.into();
            lhs_bitset ^= rhs_bitset;
            prop_assert_eq!(lhs_bitset.to_u8(), lhs.bitxor(rhs));
        }
    }

    proptest! {
        #[test]
        fn shl(bits: u8) {
            let bitset: SizedBitset<8> = bits.into();
            for i in 0..8 {
                prop_assert_eq!(bitset.shl(i).to_u8(), bits.shl(i));
            }
        }
    }

    proptest! {
        #[test]
        fn shl_assign(bits: u8) {
            let bitset: SizedBitset<8> = bits.into();
            for i in 0..8 {
                let mut bitset = bitset;
                bitset <<= i;
                prop_assert_eq!(bitset.to_u8(), bits.shl(i));
            }
        }
    }

    proptest! {
        #[test]
        fn shr(bits: u8) {
            let bitset: SizedBitset<8> = bits.into();
            for i in 0..8 {
                prop_assert_eq!(bitset.shr(i).to_u8(), bits.shr(i));
            }
        }
    }

    proptest! {
        #[test]
        fn shr_assign(bits: u8) {
            let bitset: SizedBitset<8> = bits.into();
            for i in 0..8 {
                let mut bitset = bitset;
                bitset >>= i;
                prop_assert_eq!(bitset.to_u8(), bits.shr(i));
            }
        }
    }

    proptest! {
        #[test]
        fn shl_overflow(bits: u8) {
            let bitset: SizedBitset<8> = bits.into();
            prop_assert!(bitset.shl(9).none());
        }
    }

    proptest! {
        #[test]
        fn shr_overflow(bits: u8) {
            let bitset: SizedBitset<8> = bits.into();
            prop_assert!(bitset.shr(9).none());
        }
    }

    proptest! {
        #[test]
        fn shl_assign_overflow(bits: u8) {
            let mut bitset: SizedBitset<8> = bits.into();
            bitset.shl_assign(9);
            prop_assert!(bitset.none());
        }
    }

    proptest! {
        #[test]
        fn shr_assign_overflow(bits: u8) {
            let mut bitset: SizedBitset<8> = bits.into();
            bitset.shr_assign(9);
            prop_assert!(bitset.none());
        }
    }

    proptest! {
        #[test]
        fn to_u8(bits: u8) {
            let bitset: SizedBitset<8> = bits.into();
            prop_assert_eq!(bitset.to_u8(), bits);
        }
    }

    proptest! {
        #[test]
        fn to_u16(bits: u16) {
            let bitset: SizedBitset<16> = bits.into();
            prop_assert_eq!(bitset.to_u16(), bits);
        }
    }

    proptest! {
        #[test]
        fn to_u32(bits: u32) {
            let bitset: SizedBitset<32> = bits.into();
            prop_assert_eq!(bitset.to_u32(), bits);
        }
    }

    proptest! {
        #[test]
        fn to_u64(bits: u64) {
            let bitset: SizedBitset<64> = bits.into();
            prop_assert_eq!(bitset.to_u64(), bits);
        }
    }

    proptest! {
        #[test]
        fn to_u128(bits: u128) {
            let bitset: SizedBitset<128> = bits.into();
            prop_assert_eq!(bitset.to_u128(), bits);
        }
    }
}
