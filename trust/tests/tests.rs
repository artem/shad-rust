use trust::{
    CheatingAgent, CooperatingAgent, CopycatAgent, DetectiveAgent, Game, GrudgerAgent, RoundOutcome,
};

fn test_game<'a>(mut game: Game, expected_outcomes: impl IntoIterator<Item = &'a RoundOutcome>) {
    let mut left_score = 0;
    let mut right_score = 0;

    for (i, expected) in expected_outcomes.into_iter().enumerate() {
        let outcome = game.play_round();
        let expected = expected.to_owned();
        assert_eq!(
            expected,
            outcome,
            "move #{}: expected {:?}, got {:?}",
            i + 1,
            expected,
            outcome,
        );

        match outcome {
            RoundOutcome::BothCooperated => {
                left_score += 2;
                right_score += 2;
            }
            RoundOutcome::LeftCheated => {
                left_score += 3;
                right_score -= 1;
            }
            RoundOutcome::RightCheated => {
                left_score -= 1;
                right_score += 3;
            }
            RoundOutcome::BothCheated => (),
        }

        assert_eq!(left_score, game.left_score());
        assert_eq!(right_score, game.right_score());
    }
}

#[test]
fn test_cooperators() {
    let game = Game::new(
        Box::new(CooperatingAgent::new()),
        Box::new(CooperatingAgent::new()),
    );
    test_game(game, &[RoundOutcome::BothCooperated; 12]);
}

#[test]
fn test_cheaters() {
    let game = Game::new(
        Box::new(CheatingAgent::new()),
        Box::new(CheatingAgent::new()),
    );
    test_game(game, &[RoundOutcome::BothCheated; 8]);
}

#[test]
fn test_grudgers() {
    let game = Game::new(Box::new(GrudgerAgent::new()), Box::new(GrudgerAgent::new()));
    test_game(game, &[RoundOutcome::BothCooperated; 15]);
}

#[test]
fn test_copycats() {
    let game = Game::new(Box::new(CopycatAgent::new()), Box::new(CopycatAgent::new()));
    test_game(game, &[RoundOutcome::BothCooperated; 14]);
}

#[test]
fn test_detectives() {
    let game = Game::new(
        Box::new(DetectiveAgent::new()),
        Box::new(DetectiveAgent::new()),
    );
    test_game(
        game,
        [RoundOutcome::BothCooperated; 1]
            .iter()
            .chain([RoundOutcome::BothCheated; 1].iter())
            .chain([RoundOutcome::BothCooperated; 12].iter()),
    );
}

#[test]
fn test_cooperator_cheater() {
    let game = Game::new(
        Box::new(CooperatingAgent::new()),
        Box::new(CheatingAgent::new()),
    );
    test_game(game, &[RoundOutcome::RightCheated; 18]);
}

#[test]
fn test_cooperator_grudger() {
    let game = Game::new(
        Box::new(CooperatingAgent::new()),
        Box::new(GrudgerAgent::new()),
    );
    test_game(game, &[RoundOutcome::BothCooperated; 16]);
}

#[test]
fn test_cooperator_copycat() {
    let game = Game::new(
        Box::new(CooperatingAgent::new()),
        Box::new(CopycatAgent::new()),
    );
    test_game(game, &[RoundOutcome::BothCooperated; 11]);
}

#[test]
fn test_cooperator_detective() {
    let game = Game::new(
        Box::new(CooperatingAgent::new()),
        Box::new(DetectiveAgent::new()),
    );
    test_game(
        game,
        [RoundOutcome::BothCooperated; 1]
            .iter()
            .chain([RoundOutcome::RightCheated; 1].iter())
            .chain([RoundOutcome::BothCooperated; 2].iter())
            .chain([RoundOutcome::RightCheated; 8].iter()),
    );
}

#[test]
fn test_cheater_grudger() {
    let game = Game::new(
        Box::new(CheatingAgent::new()),
        Box::new(GrudgerAgent::new()),
    );
    test_game(
        game,
        [RoundOutcome::LeftCheated; 1]
            .iter()
            .chain([RoundOutcome::BothCheated; 10].iter()),
    );
}

#[test]
fn test_cheater_copycat() {
    let game = Game::new(
        Box::new(CheatingAgent::new()),
        Box::new(CopycatAgent::new()),
    );
    test_game(
        game,
        [RoundOutcome::LeftCheated; 1]
            .iter()
            .chain([RoundOutcome::BothCheated; 7].iter()),
    );
}

#[test]
fn test_cheater_detective() {
    let game = Game::new(
        Box::new(CheatingAgent::new()),
        Box::new(DetectiveAgent::new()),
    );
    test_game(
        game,
        [RoundOutcome::LeftCheated; 1]
            .iter()
            .chain([RoundOutcome::BothCheated; 1].iter())
            .chain([RoundOutcome::LeftCheated; 2].iter())
            .chain([RoundOutcome::BothCheated; 8].iter()),
    );
}

#[test]
fn test_grudger_copycat() {
    let game = Game::new(Box::new(GrudgerAgent::new()), Box::new(CopycatAgent::new()));
    test_game(game, &[RoundOutcome::BothCooperated; 17]);
}

#[test]
fn test_grudger_detective() {
    let game = Game::new(
        Box::new(GrudgerAgent::new()),
        Box::new(DetectiveAgent::new()),
    );
    test_game(
        game,
        [RoundOutcome::BothCooperated; 1]
            .iter()
            .chain([RoundOutcome::RightCheated; 1].iter())
            .chain([RoundOutcome::LeftCheated; 2].iter())
            .chain([RoundOutcome::BothCheated; 8].iter()),
    );
}

#[test]
fn test_copycat_detective() {
    let game = Game::new(
        Box::new(CopycatAgent::new()),
        Box::new(DetectiveAgent::new()),
    );
    test_game(
        game,
        [RoundOutcome::BothCooperated; 1]
            .iter()
            .chain([RoundOutcome::RightCheated; 1].iter())
            .chain([RoundOutcome::LeftCheated; 1].iter())
            .chain([RoundOutcome::BothCooperated; 11].iter()),
    );
}
