#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, String};

#[contract]
pub struct VotingContract;

#[contracttype]
#[derive(Clone)]
pub struct Proposal {
    pub id: u32,
    pub title: String,
    pub yes_votes: u32,
    pub no_votes: u32,
}

#[contracttype]
pub enum DataKey {
    Proposal(u32),
    Voted(u32, Address),
}

#[contractimpl]
impl VotingContract {

    pub fn create_proposal(env: Env, id: u32, title: String) {

        let proposal = Proposal {
            id,
            title,
            yes_votes: 0,
            no_votes: 0,
        };

        env.storage().instance().set(&DataKey::Proposal(id), &proposal);
    }

    pub fn vote(env: Env, proposal_id: u32, voter: Address, vote_yes: bool) {

        voter.require_auth();

        let voted_key = DataKey::Voted(proposal_id, voter.clone());

        if env.storage().instance().has(&voted_key) {
            panic!("Already voted");
        }

        let mut proposal: Proposal = env
            .storage()
            .instance()
            .get(&DataKey::Proposal(proposal_id))
            .unwrap();

        if vote_yes {
            proposal.yes_votes += 1;
        } else {
            proposal.no_votes += 1;
        }

        env.storage().instance().set(&DataKey::Proposal(proposal_id), &proposal);
        env.storage().instance().set(&voted_key, &true);
    }

    pub fn get_result(env: Env, proposal_id: u32) -> Proposal {

        env.storage()
            .instance()
            .get(&DataKey::Proposal(proposal_id))
            .unwrap()
    }
}
stellar contract invoke \
  --id CBIMW74RF67T5D2XMCHI5MYBLDBJYBRGVMMDRXZSCZPILK6CY5MURMIY \
  --source-account student \
  --network testnet \
  --send=yes \
  -- create_proposal \
  --id 1 \
  --title "Community funding proposal"