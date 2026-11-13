mod combatant;
mod instance;
mod outcome;
pub mod state;
mod turn;

use combatant::{Combatant, DamageOutcomes};
pub use instance::Battle;
pub use outcome::BattleOutcome;
use turn::{AiChoices, Branch, PendingAction, Side, TurnAction, TurnContext};
