#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

use crate::RoundOutcome::{BothCheated, BothCooperated, LeftCheated, RightCheated};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundOutcome {
    BothCooperated,
    LeftCheated,
    RightCheated,
    BothCheated,
}

pub trait Agent {
    fn play_round(&self) -> bool;
    fn tell_result(&mut self, _opponent_cheated: bool) {}
}

pub struct Game {
    left_player: Box<dyn Agent>,
    right_player: Box<dyn Agent>,
    left_score: i32,
    right_score: i32,
}

impl Game {
    pub fn new(left: Box<dyn Agent>, right: Box<dyn Agent>) -> Self {
        Self {
            left_player: left,
            right_player: right,
            left_score: 0,
            right_score: 0
        }
    }

    pub fn left_score(&self) -> i32 {
        self.left_score
    }

    pub fn right_score(&self) -> i32 {
        self.right_score
    }

    pub fn play_round(&mut self) -> RoundOutcome {
        let left_cheated = self.left_player.play_round();
        let right_cheated = self.right_player.play_round();

        if !left_cheated {
            self.left_score -= 1;
            self.right_score += 3;
        }
        if !right_cheated {
            self.right_score -= 1;
            self.left_score += 3;
        }

        self.left_player.tell_result(right_cheated);
        self.right_player.tell_result(left_cheated);

        match (left_cheated, right_cheated) {
            (true, true) => BothCheated,
            (true, false) => LeftCheated,
            (false, true) => RightCheated,
            (false, false) => BothCooperated,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CheatingAgent {}

impl Agent for CheatingAgent {
    fn play_round(&self) -> bool {
        true
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CooperatingAgent {}

impl Agent for CooperatingAgent {
    fn play_round(&self) -> bool {
        false
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct GrudgerAgent {
    ever_cheated: bool,
}

impl Default for GrudgerAgent{
    fn default() -> Self {
        Self { ever_cheated: false }
    }
}

impl Agent for GrudgerAgent {
    fn play_round(&self) -> bool {
        self.ever_cheated
    }

    fn tell_result(&mut self, opponent_cheated: bool) {
        if opponent_cheated {
            self.ever_cheated = true;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct CopycatAgent {
    prev_cheated: bool
}

impl Default for CopycatAgent {
    fn default() -> Self {
        Self { prev_cheated: false }
    }
}

impl Agent for CopycatAgent {
    fn play_round(&self) -> bool {
        self.prev_cheated
    }

    fn tell_result(&mut self, opponent_cheated: bool) {
        self.prev_cheated = opponent_cheated;
    }
}

////////////////////////////////////////////////////////////////////////////////

enum DetectiveStates {
    ProbeCooperated(u8),
    ProbeCheated(u8),
    BehaviourCopycat(bool),
    BehaviourCheating,
}

fn should_cheat(num: u8) -> bool {
    num == 1
}

pub struct DetectiveAgent {
    state: DetectiveStates,
}

impl Default for DetectiveAgent {
    fn default() -> Self {
        Self { state: DetectiveStates::ProbeCooperated(0) }
    }
}

impl Agent for DetectiveAgent {
    fn play_round(&self) -> bool {
        match self.state {
            DetectiveStates::ProbeCooperated(num) => should_cheat(num),
            DetectiveStates::ProbeCheated(num) => should_cheat(num),
            DetectiveStates::BehaviourCopycat(prev) => prev,
            DetectiveStates::BehaviourCheating => true,
        }
    }

    fn tell_result(&mut self, opponent_cheated: bool) {
        self.state = match self.state {
            DetectiveStates::ProbeCooperated(num) => {
                if opponent_cheated {
                    self.state = DetectiveStates::ProbeCheated(num);
                    return self.tell_result(opponent_cheated);
                }
                if num == 3 {
                    DetectiveStates::BehaviourCheating
                } else {
                    DetectiveStates::ProbeCooperated(num + 1)
                }
            }
            DetectiveStates::ProbeCheated(num) => {
                if num == 3 {
                    DetectiveStates::BehaviourCopycat(opponent_cheated)
                } else {
                    DetectiveStates::ProbeCheated(num + 1)
                }
            }
            DetectiveStates::BehaviourCopycat(_) => DetectiveStates::BehaviourCopycat(opponent_cheated),
            DetectiveStates::BehaviourCheating => DetectiveStates::BehaviourCheating,
        }
    }
}
