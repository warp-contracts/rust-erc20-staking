use crate::action::{Action, ActionResult};
use crate::actions::staking::stake;
use crate::actions::staking::withdraw;
use crate::actions::staking::stake_of;
use crate::actions::staking::stake_all;
use crate::actions::staking::re_stake;
use crate::actions::evolve::evolve;
use crate::contract_utils::js_imports::{Block, Contract, log, SmartWeave, Transaction};
use crate::state::State;

pub async fn handle(current_state: State, action: Action) -> ActionResult {

    //Example of accessing functions imported from js:
    log("log from contract");
    log(&("Transaction::id()".to_owned() + &Transaction::id()));
    log(&("Transaction::owner()".to_owned() + &Transaction::owner()));
    log(&("Transaction::target()".to_owned() + &Transaction::target()));

    log(&("Block::height()".to_owned() + &Block::height().to_string()));
    log(&("Block::indep_hash()".to_owned() + &Block::indep_hash()));
    log(&("Block::timestamp()".to_owned() + &Block::timestamp().to_string()));

    log(&("Contract::id()".to_owned() + &Contract::id()));
    log(&("Contract::owner()".to_owned() + &Contract::owner()));

    log(&("SmartWeave::caller()".to_owned() + &SmartWeave::caller()));

    match action {
        Action::Stake { amount } => stake(current_state, amount).await,
        Action::StakeAll { } => stake_all(current_state).await,
        Action::ReStake { } => re_stake(current_state).await,
        Action::Withdraw { amount } => withdraw(current_state, amount).await,
        Action::StakeOf { target } => stake_of(current_state, target),
        Action::Evolve { value } => evolve(current_state, value),
    }
}
