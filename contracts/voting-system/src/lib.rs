#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, String, Symbol, Vec,
};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Candidate {
    pub id: u32,
    pub name: String,
    pub vote_count: u32,
}

#[contracttype]
#[derive(Clone)]
pub struct Voter {
    pub address: Address,
    pub has_voted: bool,
}

const CANDIDATES: Symbol = symbol_short!("CANDS");
const VOTERS: Symbol = symbol_short!("VOTERS");
const ADMIN: Symbol = symbol_short!("ADMIN");

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {

    // Initialize admin
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&ADMIN) {
            panic!("Already initialized");
        }

        admin.require_auth();

        env.storage().instance().set(&ADMIN, &admin);

        let candidates: Vec<Candidate> = Vec::new(&env);
        env.storage().instance().set(&CANDIDATES, &candidates);

        let voters: Vec<Voter> = Vec::new(&env);
        env.storage().instance().set(&VOTERS, &voters);
    }

    // Add candidate
    pub fn add_candidate(env: Env, name: String) -> String {
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();

        let mut candidates: Vec<Candidate> = env
            .storage()
            .instance()
            .get(&CANDIDATES)
            .unwrap_or(Vec::new(&env));

        let candidate = Candidate {
            id: candidates.len() as u32 + 1,
            name,
            vote_count: 0,
        };

        candidates.push_back(candidate);

        env.storage().instance().set(&CANDIDATES, &candidates);

        String::from_str(&env, "Candidate added")
    }

    // Vote candidate
    pub fn vote(env: Env, voter: Address, candidate_id: u32) -> String {
        voter.require_auth();

        let mut voters: Vec<Voter> = env
            .storage()
            .instance()
            .get(&VOTERS)
            .unwrap_or(Vec::new(&env));

        // Check if already voted
        for i in 0..voters.len() {
            let v = voters.get(i).unwrap();

            if v.address == voter {
                if v.has_voted {
                    panic!("Already voted");
                }
            }
        }

        let mut candidates: Vec<Candidate> = env
            .storage()
            .instance()
            .get(&CANDIDATES)
            .unwrap();

        for i in 0..candidates.len() {
            let mut candidate = candidates.get(i).unwrap();

            if candidate.id == candidate_id {
                candidate.vote_count += 1;

                candidates.set(i, candidate);

                voters.push_back(Voter {
                    address: voter,
                    has_voted: true,
                });

                env.storage().instance().set(&CANDIDATES, &candidates);
                env.storage().instance().set(&VOTERS, &voters);

                return String::from_str(&env, "Vote success");
            }
        }

        String::from_str(&env, "Candidate not found")
    }

    // Get all candidate
    pub fn get_candidates(env: Env) -> Vec<Candidate> {
        env.storage()
            .instance()
            .get(&CANDIDATES)
            .unwrap_or(Vec::new(&env))
    }
}

mod test;
