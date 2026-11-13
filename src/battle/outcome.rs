use crate::battle::Side;

#[derive(Clone)]
pub struct BattleOutcome {
    pub win: f64,
    pub lose: f64,
    pub unresolved: f64,
}

impl Default for BattleOutcome {
    fn default() -> Self {
        Self {
            win: 0.0,
            lose: 0.0,
            unresolved: 1.0,
        }
    }
}

impl BattleOutcome {
    pub(super) fn record(&mut self, winner: Side, probability: f64) {
        match winner {
            Side::Player => self.win += probability,
            Side::Ai => self.lose += probability,
        }
    }
}
