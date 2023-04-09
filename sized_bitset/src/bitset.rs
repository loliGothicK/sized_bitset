use deriving_via::DerivingVia;
use itertools::Itertools;

use crate::error::ConversionError;

/// Statically-sized Bitset
#[derive(Debug, Copy, Clone, DerivingVia)]
#[deriving(Index, IndexMut, Iter, IntoIterator, Eq, Hash)]
#[cfg_attr(any(test, feature = "arbitrary"), derive(proptest_derive::Arbitrary))]
pub struct SizedBitset<const N: usize> {
    bits: [bool; N],
}

/// const functionalities
impl<const N: usize> SizedBitset<N> {
    /// The number of bits that the bitset holds (= N).
    #[allow(unused)]
    const SIZE: usize = N;

    /// Constructs [SizedBitset] from array of bool.
    ///
    /// It's for constant initialization.
    ///
    /// # Example
    /// ```
    /// use sized_bitset::SizedBitset;
    /// const BITSET: SizedBitset<4> = SizedBitset::from_const([true, true, false, false]);
    /// ```
    pub const fn from_const(bits: [bool; N]) -> Self {
        Self { bits }
    }
}

impl<T: Into<bool> + Copy, const N: usize> From<[T; N]> for SizedBitset<N> {
    /// Make [SizedBitset] from convertible array.
    fn from(from: [T; N]) -> Self {
        Self {
            bits: from.map(|bit| bit.into()),
        }
    }
}

impl<T: Into<bool> + Copy, const N: usize> TryFrom<&[T]> for SizedBitset<N> {
    type Error = core::array::TryFromSliceError;

    /// Try make [SizedBitset] from convertible slice.
    fn try_from(from: &[T]) -> Result<Self, Self::Error> {
        Ok(Self {
            bits: from
                .iter()
                .map(|b| (*b).into())
                .collect_vec()
                .as_slice()
                .try_into()?,
        })
    }
}

impl<const N: usize> Default for SizedBitset<N> {
    /// Returns [SizedBitset] that all bits is false
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> SizedBitset<N> {
    /// Returns [SizedBitset] that all bits is false
    ///
    /// # Example
    /// ```
    /// use sized_bitset::{convert::*, SizedBitset};
    /// let bitset = SizedBitset::<8>::new();
    ///
    /// assert_eq!(bitset.to_u8(), 0b00000000);
    /// ```
    pub fn new() -> Self {
        Self {
            bits: unsafe {
                (0..N)
                    .map(|_| false)
                    .collect_vec()
                    .as_slice()
                    .try_into()
                    .unwrap_unchecked()
            },
        }
    }

    ///  Returns a String formatted as `true` => `one` and `false` => `zero`.
    ///
    /// # Example
    /// ```
    /// use sized_bitset::SizedBitset;
    /// let bitset = SizedBitset::from([true, true, false, false]);
    ///
    /// assert_eq!(bitset.to_string_with('a', 'b'), "bbaa".to_owned());
    /// ```
    pub fn to_string_with(&self, one: char, zero: char) -> String {
        self.bits
            .map(|bit| if bit { one } else { zero })
            .iter()
            .rev()
            .collect::<String>()
    }
}

impl<const N: usize> core::str::FromStr for SizedBitset<N> {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.chars()
            .map(|bit| match bit {
                '0' => Ok(false),
                '1' => Ok(true),
                _ => Err(ConversionError::FromStr(s.to_owned())),
            })
            .rev()
            .collect::<Result<Vec<_>, _>>()?
            .as_slice()
            .try_into()?)
    }
}

impl<const N: usize> core::fmt::Display for SizedBitset<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            self.bits
                .map(|bit| if bit { '1' } else { '0' })
                .iter()
                .rev()
                .collect::<String>()
        )
    }
}

/// Element access
impl<const N: usize> SizedBitset<N> {
    /// Checks if all bits are set to true.
    ///
    /// # Example
    /// ```
    /// use sized_bitset::SizedBitset;
    /// let bitset = SizedBitset::from_const([true, true, true, true]);
    /// assert!(bitset.all());
    /// ```
    pub fn all(&self) -> bool {
        self.bits.iter().all(|bit| *bit)
    }

    /// Checks if any bits are set to true.
    ///
    /// # Example
    /// ```
    /// use sized_bitset::SizedBitset;
    /// let bitset = SizedBitset::from_const([false, false, true, false]);
    /// assert!(bitset.any());
    /// ```
    pub fn any(&self) -> bool {
        self.bits.iter().any(|bit| *bit)
    }

    /// Checks if none bits are set to true.
    ///
    /// # Example
    /// ```
    /// use sized_bitset::SizedBitset;
    /// let bitset = SizedBitset::from_const([false, false, false, false]);
    /// assert!(bitset.none());
    /// ```
    pub fn none(&self) -> bool {
        self.bits.iter().all(|bit| !*bit)
    }

    /// Returns the number of bits set to true.
    ///
    /// ```
    /// use sized_bitset::SizedBitset;
    /// let bitset = SizedBitset::from_const([true, false, true, false]);
    /// assert_eq!(bitset.count(), 2);
    /// ```
    pub fn count(&self) -> usize {
        self.bits.iter().filter(|bit| **bit).count()
    }
}

/// Modifiers
impl<const N: usize> SizedBitset<N> {
    /// Returns [SizedBitset] that all bits are flipped.
    ///
    /// # Example
    /// ```
    /// use sized_bitset::SizedBitset;
    /// let bitset = SizedBitset::from_const([false, false, false, false]);
    /// assert_eq!(bitset.flipped(), SizedBitset::from_const([true, true, true, true]));
    /// ```
    pub fn flipped(&self) -> Self {
        Self {
            bits: self.bits.map(std::ops::Not::not),
        }
    }

    /// Flips all bits.
    ///
    /// ```
    /// use sized_bitset::SizedBitset;
    /// let mut bitset = SizedBitset::from_const([false, false, false, false]);
    /// bitset.flip();
    /// assert_eq!(bitset, SizedBitset::from_const([true, true, true, true]));
    /// ```
    pub fn flip(&mut self) {
        self.bits = self.bits.map(std::ops::Not::not);
    }

    /// Sets the bit for the specified index to true.
    ///
    /// ```
    /// use sized_bitset::SizedBitset;
    /// let mut bitset = SizedBitset::from_const([false, false, false, false]);
    /// bitset.set(1);
    /// assert_eq!(bitset, SizedBitset::from_const([false, true, false, false]));
    /// ```
    pub fn set(&mut self, index: usize) {
        self.bits[index] = true;
    }

    /// Sets all bits to true.
    ///
    /// ```
    /// use sized_bitset::SizedBitset;
    /// let mut bitset = SizedBitset::from_const([false, false, false, false]);
    /// bitset.set_all();
    /// assert_eq!(bitset, SizedBitset::from_const([true, true, true, true]));
    /// ```
    pub fn set_all(&mut self) {
        self.bits = self.bits.map(|_| true);
    }

    /// Sets the bit for the specified index to false.
    ///
    /// ```
    /// use sized_bitset::SizedBitset;
    /// let mut bitset = SizedBitset::from_const([true, true, true, true]);
    /// bitset.reset(1);
    /// assert_eq!(bitset, SizedBitset::from_const([true, false, true, true]));
    /// ```
    pub fn reset(&mut self, index: usize) {
        self.bits[index] = false;
    }

    /// Sets all bits to false.
    ///
    /// ```
    /// use sized_bitset::SizedBitset;
    /// let mut bitset = SizedBitset::from_const([true, true, true, true]);
    /// bitset.reset_all();
    /// assert_eq!(bitset, SizedBitset::from_const([false, false, false, false]));
    /// ```
    pub fn reset_all(&mut self) {
        self.bits = self.bits.map(|_| false);
    }

    /// Computes the result of bitwise left-rotating the bits of `self` by `s` positions.
    /// This operation is also known as a [left circular shift](https://en.wikipedia.org/wiki/Circular_shift).
    ///
    /// ```
    /// use sized_bitset::convert::To8;
    /// use sized_bitset::SizedBitset;
    /// let bitset: SizedBitset<8> = 0b00011101.into();
    /// assert_eq!(bitset.rotl(2).to_u8(), 0b01110100);
    /// ```
    pub fn rotl(&self, s: usize) -> Self {
        let r = s % N;

        if r == 0 {
            *self
        } else {
            (*self << r) | (*self >> (N - r))
        }
    }

    /// Computes the result of bitwise right-rotating the bits of `self` by `s` positions.
    /// This operation is also known as a [right circular shift](https://en.wikipedia.org/wiki/Circular_shift).
    ///
    /// ```
    /// use sized_bitset::convert::To8;
    /// use sized_bitset::SizedBitset;
    /// let bitset: SizedBitset<8> = 0b00011101.into();
    /// assert_eq!(bitset.rotr(2).to_u8(), 0b01000111);
    /// ```
    pub fn rotr(&self, s: usize) -> Self {
        let r = s % N;

        if r == 0 {
            *self
        } else {
            (*self >> r) | (*self << (N - r))
        }
    }
}

impl<const N: usize> core::ops::BitAnd for SizedBitset<N> {
    type Output = Self;

    /// Returns a [SizedBitset] containing the result of binary XOR on corresponding pairs of bits of `self` and `rhs`.
    fn bitand(self, rhs: Self) -> Self::Output {
        unsafe {
            (0..N)
                .map(|i| self.bits[i].bitand(rhs.bits[i]))
                .collect_vec()
                .as_slice()
                .try_into()
                .unwrap_unchecked()
        }
    }
}

impl<const N: usize> core::ops::BitOr for SizedBitset<N> {
    type Output = Self;

    /// Returns a [SizedBitset] containing the result of binary XOR on corresponding pairs of bits of `self` and `rhs`.
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe {
            (0..N)
                .map(|i| self.bits[i].bitor(rhs.bits[i]))
                .collect_vec()
                .as_slice()
                .try_into()
                .unwrap_unchecked()
        }
    }
}

impl<const N: usize> core::ops::BitXor for SizedBitset<N> {
    type Output = Self;

    /// Returns a [SizedBitset] containing the result of binary XOR on corresponding pairs of bits of `self` and `rhs`.
    fn bitxor(self, rhs: Self) -> Self::Output {
        unsafe {
            (0..N)
                .map(|i| self.bits[i].bitxor(rhs.bits[i]))
                .collect_vec()
                .as_slice()
                .try_into()
                .unwrap_unchecked()
        }
    }
}

impl<const N: usize> core::ops::BitAndAssign for SizedBitset<N> {
    /// Sets the bits to the result of binary AND on corresponding pairs of bits of `self` and `other`.
    fn bitand_assign(&mut self, other: Self) {
        for i in 0..N {
            self.bits[i].bitand_assign(other.bits[i]);
        }
    }
}

impl<const N: usize> core::ops::BitOrAssign for SizedBitset<N> {
    /// Sets the bits to the result of binary OR on corresponding pairs of bits of `self` and `other`.
    fn bitor_assign(&mut self, other: Self) {
        for i in 0..N {
            self.bits[i].bitor_assign(other.bits[i]);
        }
    }
}

impl<const N: usize> core::ops::BitXorAssign for SizedBitset<N> {
    /// Sets the bits to the result of binary XOR on corresponding pairs of bits of `self` and `other`.
    fn bitxor_assign(&mut self, other: Self) {
        for i in 0..N {
            self.bits[i].bitxor_assign(other.bits[i]);
        }
    }
}

impl<const N: usize> core::ops::Shl<usize> for SizedBitset<N> {
    type Output = Self;

    /// Performs binary shift right.
    ///
    /// # Example
    /// ```
    /// use sized_bitset::convert::To8;
    /// use sized_bitset::SizedBitset;
    ///
    /// let bitset: SizedBitset<8> = 0b01110010.into();
    /// assert_eq!((bitset << 1).to_u8(), 0b11100100);
    /// ```
    fn shl(self, rhs: usize) -> Self::Output {
        if N <= rhs {
            SizedBitset::new()
        } else {
            unsafe {
                (0..rhs)
                    .map(|_| false)
                    .chain(self.bits[0..(N - rhs)].iter().cloned())
                    .collect_vec()
                    .as_slice()
                    .try_into()
                    .unwrap_unchecked()
            }
        }
    }
}

impl<const N: usize> core::ops::Shr<usize> for SizedBitset<N> {
    type Output = Self;

    /// Performs binary shift left.
    ///
    /// # Example
    /// ```
    /// use sized_bitset::convert::To8;
    /// use sized_bitset::SizedBitset;
    ///
    /// let bitset: SizedBitset<8> = 0b01110010.into();
    /// assert_eq!((bitset >> 1).to_u8(), 0b00111001);
    /// ```
    fn shr(self, rhs: usize) -> Self::Output {
        if N <= rhs {
            SizedBitset::new()
        } else {
            unsafe {
                self.bits[rhs..]
                    .iter()
                    .cloned()
                    .chain((0..rhs).map(|_| false))
                    .collect_vec()
                    .as_slice()
                    .try_into()
                    .unwrap_unchecked()
            }
        }
    }
}

impl<const N: usize> core::ops::ShlAssign<usize> for SizedBitset<N> {
    /// Performs binary shift left and modify `self`.
    ///
    /// # Example
    /// ```
    /// use sized_bitset::convert::To8;
    /// use sized_bitset::SizedBitset;
    ///
    /// let mut bitset: SizedBitset<8> = 0b01110011.into();
    /// bitset <<= 2;
    /// assert_eq!(bitset.to_u8(), 0b11001100);
    /// ```
    fn shl_assign(&mut self, rhs: usize) {
        if rhs == 0 {
            return;
        }

        if N <= rhs {
            self.reset_all();
        } else {
            for i in (rhs..N).rev() {
                self.bits[i] = self.bits[i - rhs];
            }
            for i in 0..rhs {
                self.bits[i] = false;
            }
        }
    }
}

impl<const N: usize> core::ops::ShrAssign<usize> for SizedBitset<N> {
    /// Performs binary shift right and modify `self`.
    ///
    /// # Example
    /// ```
    /// use sized_bitset::convert::To8;
    /// use sized_bitset::SizedBitset;
    ///
    /// let mut bitset: SizedBitset<8> = 0b01110010.into();
    /// bitset >>= 2;
    /// assert_eq!(bitset.to_u8(), 0b00011100);
    /// ```
    fn shr_assign(&mut self, rhs: usize) {
        if rhs == 0 {
            return;
        }

        if N <= rhs {
            self.reset_all();
        } else {
            for i in rhs..N {
                self.bits[i - rhs] = self.bits[i];
            }
            for i in (N - rhs)..N {
                self.bits[i] = false;
            }
        }
    }
}

pub mod convert {
    pub trait To8 {
        fn to_u8(&self) -> u8;
    }
    pub trait To16 {
        fn to_u16(&self) -> u16;
    }
    pub trait To32 {
        fn to_u32(&self) -> u32;
    }
    pub trait To64 {
        fn to_u64(&self) -> u64;
    }
    pub trait To128 {
        fn to_u128(&self) -> u128;
    }
}

sized_bitset_macros::generate_num_traits!();
