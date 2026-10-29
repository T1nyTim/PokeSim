use core::fmt::{self, Formatter};

use crate::{battle::BattleOutcome, matchup::MatchupKind};

#[derive(Default)]
pub struct WinRates {
    overall: EncounterResults,
    trainer: EncounterResults,
    bosses: EncounterResults,
}

impl fmt::Display for WinRates {
    #[expect(clippy::question_mark_used, reason = "will allow for core lib code")]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (label, results) in [("Overall", &self.overall), ("Trainer", &self.trainer), ("Bosses", &self.bosses)] {
            let percent = 100.0f64 / results.encounters as f64;
            writeln!(
                f,
                "{label} ({} Pokemon): win {:.10}%, lose {:.10}%, unresolved {:.10}%",
                results.encounters,
                results.win * percent,
                results.lose * percent,
                results.unresolved * percent
            )?;
        }
        Ok(())
    }
}

impl WinRates {
    pub fn record(&mut self, kind: &MatchupKind, outcome: &BattleOutcome) {
        self.overall.record(outcome);
        match *kind {
            MatchupKind::Boss => {
                self.trainer.record(outcome);
                self.bosses.record(outcome);
            }
            MatchupKind::Trainer => self.trainer.record(outcome),
            MatchupKind::Wild => {}
        }
    }
}

#[derive(Default)]
struct EncounterResults {
    encounters: usize,
    win: f64,
    lose: f64,
    unresolved: f64,
}

impl EncounterResults {
    fn record(&mut self, outcome: &BattleOutcome) {
        self.encounters += 1;
        self.win += outcome.win;
        self.lose += outcome.lose;
        self.unresolved += outcome.unresolved;
    }
}
