#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundOutcome {
    BothCooperated,
    LeftCheated,
    RightCheated,
    BothCheated,
}

pub struct Game {
    // TODO: your code here.
}

impl Game {
    // pub fn new(left: Box<???>, right: Box<???>) -> Self {
    // TODO: your code here.
    // }

    pub fn left_score(&self) -> i32 {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn right_score(&self) -> i32 {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn play_round(&mut self) -> RoundOutcome {
        // TODO: your code here.
        unimplemented!()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CheatingAgent {}

// TODO: your code here.

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CooperatingAgent {}

// TODO: your code here.

////////////////////////////////////////////////////////////////////////////////

pub struct GrudgerAgent {
    // TODO: your code here.
}

// TODO: your code here.

////////////////////////////////////////////////////////////////////////////////

pub struct CopycatAgent {
    // TODO: your code here.
}

// TODO: your code here.

////////////////////////////////////////////////////////////////////////////////

pub struct DetectiveAgent {
    // TODO: your code here.
}

// TODO: your code here.
