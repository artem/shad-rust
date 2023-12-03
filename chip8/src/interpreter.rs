use std::ops::{Index, IndexMut};

use crate::{
    data::{Address, Nibble, OpCode, RegisterIndex, Word},
    image::Image,
    platform::{Platform, Point, Sprite},
    Error, Offset, Result,
};

////////////////////////////////////////////////////////////////////////////////

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

////////////////////////////////////////////////////////////////////////////////

pub struct Interpreter<P: Platform> {
    platform: P,
    // TODO: your code here.
}

impl<P: Platform> Interpreter<P> {
    pub fn new(image: impl Image, platform: P) -> Self {
        Self {
            platform,
            // TODO: your code here.
        }
    }

    pub fn platform(&self) -> &P {
        &self.platform
    }

    pub fn platform_mut(&mut self) -> &mut P {
        &mut self.platform
    }

    pub fn run_next_instruction(&mut self) -> Result<()> {
        // TODO: your code here.
        unimplemented!()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    ClearScreen,
    Return,
    Jump(Address),
    Call(Address),
    SkipIfEqual(RegisterIndex, Word),
    SkipIfNotEqual(RegisterIndex, Word),
    SkipIfRegistersEqual(RegisterIndex, RegisterIndex),
    SetRegister(RegisterIndex, Word),
    AddValue(RegisterIndex, Word),
    SetToRegister(RegisterIndex, RegisterIndex),
    Or(RegisterIndex, RegisterIndex),
    And(RegisterIndex, RegisterIndex),
    Xor(RegisterIndex, RegisterIndex),
    AddRegister(RegisterIndex, RegisterIndex),
    SubRegister(RegisterIndex, RegisterIndex),
    ShiftRight(RegisterIndex, RegisterIndex),
    SubRegisterReversed(RegisterIndex, RegisterIndex),
    ShiftLeft(RegisterIndex, RegisterIndex),
    SkipIfRegistersNotEqual(RegisterIndex, RegisterIndex),
    SetIndexRegister(Address),
    JumpV0(Address),
    SetToRandom(RegisterIndex, Word),
    Draw(RegisterIndex, RegisterIndex, Nibble),
    SkipIfKeyDown(RegisterIndex),
    SkipIfKeyUp(RegisterIndex),
    GetDelayTimer(RegisterIndex),
    WaitForKey(RegisterIndex),
    SetDelayTimer(RegisterIndex),
    SetSoundTimer(RegisterIndex),
    IncrementIndexRegister(RegisterIndex),
    SetIndexRegisterToSprite(Nibble),
    ToDecimal(RegisterIndex),
    WriteMemory(Nibble),
    ReadMemory(Nibble),
}

impl TryFrom<OpCode> for Operation {
    type Error = ();

    fn try_from(code: OpCode) -> std::result::Result<Self, ()> {
        let op = match code.as_u16() {
            // TODO: your code here.
            _ => return Err(()),
        };
        Ok(op)
    }
}

////////////////////////////////////////////////////////////////////////////////
