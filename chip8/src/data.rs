use std::{
    fmt::{Display, Formatter},
    ops::{Add, AddAssign},
};

pub type Word = u8;
pub type Offset = i16;
pub type RegisterIndex = Nibble;

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Nibble(u8);

impl Nibble {
    pub const WIDTH: usize = 4;
    pub const DOMAIN_SIZE: usize = 2usize.pow(Self::WIDTH as u32);
    pub const MAX: Nibble = Nibble((Self::DOMAIN_SIZE - 1) as u8);

    pub fn as_u8(self) -> u8 {
        self.0
    }

    pub fn as_usize(self) -> usize {
        self.0 as usize
    }

    pub fn as_offset(self) -> Offset {
        self.0 as Offset
    }
}

impl TryFrom<u8> for Nibble {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= Self::MAX.0 {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

impl From<Nibble> for usize {
    fn from(value: Nibble) -> Self {
        value.0 as usize
    }
}

impl Display for Nibble {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:#03x}", self.0)
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Default)]
pub struct Address(u16);

impl Address {
    pub const WIDTH: usize = 12;
    pub const DOMAIN_SIZE: usize = 2usize.pow(Self::WIDTH as u32);
    pub const MAX: Address = Address((Self::DOMAIN_SIZE - 1) as u16);

    pub const fn new(value: u16) -> Self {
        debug_assert!(
            (value as usize) < Self::DOMAIN_SIZE,
            "chip8 address value overflow"
        );
        Self(value % Self::DOMAIN_SIZE as u16)
    }

    pub fn as_usize(self) -> usize {
        self.0 as usize
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Add<Offset> for Address {
    type Output = Address;

    fn add(self, rhs: Offset) -> Self::Output {
        Address(self.0.wrapping_add_signed(rhs) % Self::DOMAIN_SIZE as u16)
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl AddAssign<Offset> for Address {
    fn add_assign(&mut self, rhs: Offset) {
        self.0 = self.0.wrapping_add_signed(rhs) % Self::DOMAIN_SIZE as u16;
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:#06x}", self.0)
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug)]
pub struct OpCode(u16);

impl OpCode {
    pub fn from_bytes(big: u8, little: u8) -> Self {
        Self(((big as u16) << 8) + little as u16)
    }

    pub fn extract_address(self) -> Address {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn extract_word(self, index: usize) -> Word {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn extract_nibble(self, index: usize) -> Nibble {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn as_u16(self) -> u16 {
        self.0
    }
}

impl From<OpCode> for u16 {
    fn from(value: OpCode) -> Self {
        value.0
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:#06x}", self.0)
    }
}
