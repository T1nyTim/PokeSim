use crate::{
    battle::{Combatant, Side},
    data::{Move, Type, effects::Effect, moves::BIDE},
};

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct BideState {
    turns_left: u8,
    damage: u16,
}

impl BideState {
    pub(crate) const fn new(turns_left: u8) -> Self {
        Self { turns_left, damage: 0 }
    }

    pub(super) const fn advance(&mut self, last_damage: u16) -> Option<u16> {
        self.damage = self.damage.wrapping_add(last_damage);
        self.turns_left -= 1;
        if self.turns_left == 0 { Some(self.damage.wrapping_mul(2)) } else { None }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct CombatantState {
    pub hp: u16,
    pub stages: StatStages,
    bide: Option<BideState>,
    confusion_elapsed: Option<u8>,
    disabled: Option<DisableState>,
    poisoned: bool,
    seeded: bool,
    sleep_elapsed: Option<u8>,
    trapping: Option<TrappingState>,
}

impl CombatantState {
    pub(crate) const fn poison(&mut self) {
        self.poisoned = true;
    }

    pub(crate) fn can_be_poisoned(&self, types: &[Type; 2]) -> bool {
        self.hp > 0 && !self.has_status_condition() && !types.contains(&Type::Poison)
    }

    pub(super) const fn can_be_disabled(&self) -> bool {
        self.hp > 0 && self.disabled.is_none()
    }

    pub(super) const fn can_be_put_to_sleep(&self) -> bool {
        self.hp > 0 && !self.has_status_condition()
    }

    pub(super) const fn confuse(&mut self) {
        if self.hp > 0 && self.confusion_elapsed.is_none() {
            self.confusion_elapsed = Some(0);
        }
    }

    pub(super) const fn disable(&mut self, disabled: DisableState) {
        self.disabled = Some(disabled);
    }

    pub(super) const fn end_confusion(&mut self) {
        self.confusion_elapsed = None;
    }

    pub(super) const fn end_disable(&mut self) {
        self.disabled = None;
    }

    pub(super) const fn end_trapping(&mut self) {
        self.trapping.as_mut().expect("Ending a trapping move requires an active trap").end();
    }

    pub(super) const fn fall_asleep(&mut self) {
        self.sleep_elapsed = Some(0);
    }

    pub(super) const fn has_status_condition(&self) -> bool {
        self.poisoned || self.sleep_elapsed.is_some()
    }

    pub(super) const fn is_asleep(&self) -> bool {
        self.sleep_elapsed.is_some()
    }

    pub(super) const fn wake_up(&mut self) {
        self.sleep_elapsed = None;
    }

    pub(super) fn advance_confusion(&mut self) -> Option<f64> {
        #[expect(clippy::question_mark_used, reason = "handled before end of function")]
        let turns_elapsed = self.confusion_elapsed.as_mut()?;
        let probability = Effect::Confusion.duration_end_probability(*turns_elapsed);
        *turns_elapsed += 1;
        Some(probability)
    }

    pub(super) fn advance_disable(&mut self) -> Option<f64> {
        self.disabled.as_mut().map(DisableState::advance)
    }

    pub(super) fn advance_sleep(&mut self) -> Option<f64> {
        #[expect(clippy::question_mark_used, reason = "handled before end of function")]
        let turns_elapsed = self.sleep_elapsed.as_mut()?;
        let probability = Effect::Sleep.duration_end_probability(*turns_elapsed);
        *turns_elapsed += 1;
        Some(probability)
    }

    pub(super) fn can_be_seeded(&self, types: &[Type; 2]) -> bool {
        !self.seeded && !types.contains(&Type::Grass)
    }

    pub(super) fn disabled_slot(&self) -> Option<u8> {
        self.disabled.as_ref().map(|disabled| disabled.slot)
    }

    pub(super) fn heal(&mut self, amount: u16, max_hp: u16) {
        self.hp = self.hp.saturating_add(amount).min(max_hp);
    }

    pub(super) fn is_disabled(&self, slot: usize) -> bool {
        self.disabled.as_ref().is_some_and(|disabled| disabled.slot == slot as u8)
    }

    pub(super) fn residual_damage(max_hp: u16) -> u16 {
        (max_hp / 16).max(1)
    }

    pub(super) fn trapping_move(&self) -> Option<&'static Move> {
        self.trapping.as_ref().map(|trapping| trapping.mv)
    }

    const fn start_bide(&mut self, bide: BideState) {
        self.bide = Some(bide);
    }

    const fn start_trapping(&mut self, mv: &'static Move, damage: u16) {
        self.trapping = Some(TrappingState::new(mv, damage));
    }

    #[expect(clippy::question_mark_used, reason = "handled before end of function")]
    fn continue_bide(&mut self, last_damage: u16) -> Option<u16> {
        let damage = self.bide.as_mut()?.advance(last_damage)?;
        self.bide = None;
        Some(damage)
    }

    fn continue_trapping(&mut self) -> (&'static Move, u16, f64) {
        self.trapping
            .as_mut()
            .expect("Continuing a trapping move requires an active trap")
            .advance()
    }

    fn finish_turn(&mut self) {
        if self.trapping.as_ref().is_some_and(TrappingState::is_finished) {
            self.trapping = None;
        }
    }

    fn hurt_in_confusion(&mut self, damage: u16) -> u16 {
        let dmg_dealt = damage.min(self.hp);
        self.hp -= dmg_dealt;
        self.bide = None;
        self.trapping = None;
        dmg_dealt
    }

    fn new(hp: u16) -> Self {
        Self {
            hp,
            stages: StatStages::default(),
            bide: None,
            confusion_elapsed: None,
            disabled: None,
            poisoned: false,
            seeded: false,
            sleep_elapsed: None,
            trapping: None,
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct DisableState {
    slot: u8,
    turns_elapsed: u8,
}

impl DisableState {
    pub(crate) const fn new(slot: usize) -> Self {
        Self {
            slot: slot as u8,
            turns_elapsed: 0,
        }
    }

    fn advance(&mut self) -> f64 {
        let probability = Effect::Disable.duration_end_probability(self.turns_elapsed);
        self.turns_elapsed += 1;
        probability
    }
}

#[derive(Clone, Default, Eq, Hash, PartialEq)]
pub struct StatStages {
    pub atk: i8,
    pub def: i8,
    pub spd: i8,
    pub acc: i8,
}

impl StatStages {
    pub(super) fn apply(stat: u16, stage: i8) -> u32 {
        let (numerator, denominator) = match stage {
            -6 => (25, 100),
            -5 => (28, 100),
            -4 => (33, 100),
            -3 => (40, 100),
            -2 => (50, 100),
            -1 => (66, 100),
            0 => (1, 1),
            1 => (15, 10),
            2 => (2, 1),
            3 => (25, 10),
            4 => (3, 1),
            5 => (35, 10),
            6 => (4, 1),
            _ => unreachable!("Stat stages must be between -6 and 6"),
        };
        u32::from(stat * numerator / denominator).clamp(1, 999)
    }

    pub(super) fn hit_probability(&self, mv: &Move) -> f64 {
        let threshold = Self::apply(u16::from(mv.accuracy()), self.acc).min(255);
        f64::from(threshold) / 256.0
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub(super) struct BattleState {
    pub player: CombatantState,
    pub ai: CombatantState,
    pub player_pp: [u8; 4],
    last_damage: u16,
}

impl BattleState {
    pub(super) fn new(player: &Combatant<'_>, ai: &Combatant<'_>) -> Self {
        Self {
            player: CombatantState::new(player.stats.hp),
            ai: CombatantState::new(ai.stats.hp),
            player_pp: player.pokemon.moves.map(|slot| slot.map_or(0, |mv| mv.pp)),
            last_damage: 0,
        }
    }

    pub(super) const fn clear_unused_damage(&mut self) {
        if self.player.bide.is_none() && self.ai.bide.is_none() {
            self.last_damage = 0;
        }
    }

    pub(super) const fn combatant(&self, side: Side) -> &CombatantState {
        match side {
            Side::Player => &self.player,
            Side::Ai => &self.ai,
        }
    }

    pub(super) const fn combatant_mut(&mut self, side: Side) -> &mut CombatantState {
        match side {
            Side::Player => &mut self.player,
            Side::Ai => &mut self.ai,
        }
    }

    pub(super) const fn is_biding(&self, side: Side) -> bool {
        match side {
            Side::Player => self.player.bide.is_some(),
            Side::Ai => self.ai.bide.is_some(),
        }
    }

    pub(super) const fn is_trapped(&self, side: Side) -> bool {
        match side {
            Side::Player => self.ai.trapping.is_some(),
            Side::Ai => self.player.trapping.is_some(),
        }
    }

    pub(super) const fn miss_move(&mut self) {
        self.last_damage = 0;
    }

    pub(super) const fn start_bide(&mut self, side: Side, bide: BideState) {
        let combatant = match side {
            Side::Player => &mut self.player,
            Side::Ai => &mut self.ai,
        };
        combatant.start_bide(bide);
    }

    pub(super) const fn start_trapping(&mut self, side: Side, mv: &'static Move, damage: u16) {
        let attacker = match side {
            Side::Player => &mut self.player,
            Side::Ai => &mut self.ai,
        };
        attacker.start_trapping(mv, damage);
    }

    pub(super) const fn winner(&self) -> Option<Side> {
        if self.player.hp == 0 {
            Some(Side::Ai)
        } else if self.ai.hp == 0 {
            Some(Side::Player)
        } else {
            None
        }
    }

    pub(super) fn apply_damage(&mut self, dmg: u16, mv: &Move, side: Side) -> u16 {
        let (attacker, target) = match side {
            Side::Player => (&mut self.player, &mut self.ai),
            Side::Ai => (&mut self.ai, &mut self.player),
        };
        let dmg_dealt = dmg.min(target.hp);
        target.hp -= dmg_dealt;
        let recoil = mv.recoil_damage(dmg_dealt);
        attacker.hp = attacker.hp.saturating_sub(recoil);
        self.last_damage = dmg_dealt;
        dmg_dealt
    }

    pub(super) fn apply_residual_damage(&mut self, side: Side, max_hp: u16, opponent_max_hp: u16) {
        let (combatant, opponent) = match side {
            Side::Player => (&mut self.player, &mut self.ai),
            Side::Ai => (&mut self.ai, &mut self.player),
        };
        let dmg = CombatantState::residual_damage(max_hp);
        if combatant.poisoned {
            combatant.hp = combatant.hp.saturating_sub(dmg);
        }
        if combatant.seeded {
            combatant.hp = combatant.hp.saturating_sub(dmg);
            opponent.heal(dmg, opponent_max_hp);
        }
    }

    pub(super) fn apply_secondary(&mut self, effect: &Effect, side: Side) {
        let target = match side {
            Side::Player => &mut self.ai,
            Side::Ai => &mut self.player,
        };
        effect.apply_secondary(target);
    }

    pub(super) fn apply_status_move(&mut self, effect: &Effect, side: Side, target_types: &[Type; 2]) {
        let target = match (side, effect.targets_user()) {
            (Side::Player, true) | (Side::Ai, false) => &mut self.player,
            (Side::Ai, true) | (Side::Player, false) => &mut self.ai,
        };
        match effect {
            Effect::Confusion => target.confuse(),
            Effect::LeechSeed => {
                if target.can_be_seeded(target_types) {
                    target.seeded = true;
                }
            }
            Effect::NoEffect
            | Effect::AccuracyDown
            | Effect::AttackDown
            | Effect::AttackTwice
            | Effect::Bide
            | Effect::BurnSide1
            | Effect::Charge
            | Effect::ConfusionSide
            | Effect::DefenseDown1
            | Effect::DefenseDown2
            | Effect::DefenseDownSide
            | Effect::DefenseUp1
            | Effect::Disable
            | Effect::DrainHp
            | Effect::EvasionUp
            | Effect::Explode
            | Effect::Flinch1
            | Effect::FocusEnergy
            | Effect::Haze
            | Effect::Heal
            | Effect::LightScreen
            | Effect::Metronome
            | Effect::MirrorMove
            | Effect::OHKO
            | Effect::Paralyze
            | Effect::ParalyzeSide2
            | Effect::Poison
            | Effect::PoisonSide1
            | Effect::Rage
            | Effect::Recoil
            | Effect::Sleep
            | Effect::SpecialUp1
            | Effect::SpeedDown
            | Effect::SpeedUp
            | Effect::SuperFang
            | Effect::Swift
            | Effect::SwitchAndTeleport
            | Effect::ThrashPetalDance
            | Effect::Trapping
            | Effect::Twineedle
            | Effect::TwoToFiveAttacks => effect.apply_status(&mut target.stages),
        }
    }

    pub(super) fn begin_move(&mut self, side: Side, slot: Option<usize>, effect: &Effect) {
        if side == Side::Player
            && let Some(idx) = slot
        {
            self.player_pp[idx] = self.player_pp[idx]
                .checked_sub(1)
                .expect("The player's selected move must have PP remaining");
        }
        if !effect.preserves_last_damage() {
            self.last_damage = 0;
        }
    }

    pub(super) fn cap_surplus_pp(&mut self, remaining_turns: usize) {
        for pp in &mut self.player_pp {
            *pp = (*pp).min(remaining_turns as u8);
        }
    }

    pub(super) fn continue_bide(&mut self, side: Side) {
        let combatant = match side {
            Side::Player => &mut self.player,
            Side::Ai => &mut self.ai,
        };
        if let Some(damage) = combatant.continue_bide(self.last_damage) {
            self.apply_damage(damage, &BIDE, side);
        }
    }

    pub(super) fn continue_trapping(&mut self, side: Side) -> f64 {
        let attacker = match side {
            Side::Player => &mut self.player,
            Side::Ai => &mut self.ai,
        };
        let (mv, damage, expiry_prob) = attacker.continue_trapping();
        self.apply_damage(damage, mv, side);
        expiry_prob
    }

    pub(super) fn finish_turn(&mut self) {
        self.player.finish_turn();
        self.ai.finish_turn();
    }

    pub(super) fn hurt_in_confusion(&mut self, side: Side, damage: u16) {
        self.last_damage = self.combatant_mut(side).hurt_in_confusion(damage);
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct TrappingState {
    mv: &'static Move,
    damage: u16,
    turns_elapsed: u8,
    ends_this_turn: bool,
}

impl TrappingState {
    const fn new(mv: &'static Move, damage: u16) -> Self {
        Self {
            mv,
            damage,
            turns_elapsed: 0,
            ends_this_turn: false,
        }
    }

    const fn end(&mut self) {
        self.ends_this_turn = true;
    }

    const fn is_finished(&self) -> bool {
        self.ends_this_turn
    }

    fn advance(&mut self) -> (&'static Move, u16, f64) {
        let expiry_prob = self.mv.effect.duration_end_probability(self.turns_elapsed);
        self.turns_elapsed += 1;
        (self.mv, self.damage, expiry_prob)
    }
}
