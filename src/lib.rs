#![feature(const_btree_new)]
#![no_std]

// External packages (crates) and internal modules import
use codec::{Decode, Encode};
use gstd::{debug, msg, prelude::*};
use scale_info::TypeInfo;

// This defines the meta information about the contract
// for the Gear IDEA portal to parse.
// It also defines the communication interface via input / output fields.
gstd::metadata! {
    title: "Voting App",
    handle:
        input: Action,
    state:
        input: StateAction,
        output: StateReply,
}

#[derive(Debug, TypeInfo, Decode)]
pub enum Action {
    AddCandidate(String),
    VoteForCandidate(String),
}

#[derive(Debug, TypeInfo, Encode)]
pub enum StateReply {
    All(BTreeMap<String, i32>),
    VotesFor(i32),
}

#[derive(Debug, TypeInfo, Decode)]
pub enum StateAction {
    All,
    VotesFor(String),
}

#[derive(Clone)]
pub struct State {
    votes_received: BTreeMap<String, i32>,
}

impl State {
    // Create a state
    pub const fn new() -> Self {
        Self {
            votes_received: BTreeMap::new(),
        }
    }

    // Add new candidate
    pub fn add_candidate(&mut self, candidate: String) {
        self.votes_received.insert(candidate, 0);
    }

    // Vote for candidate by name. If candidate no exist add it
    pub fn vote_for_candidate(&mut self, name: String) {
        let counter = self.votes_received.entry(name).or_insert(0);
        *counter += 1;
    }
}

// The state itself (i.e. the variable state will be accessed through)
static mut STATE: State = State::new();

// Init function that is executed once upon contract initialization
// Here is empty
#[no_mangle]
pub unsafe extern "C" fn init() {}

// Handle function that processes the incoming message
#[no_mangle]
pub unsafe extern "C" fn handle() {
    let action: Action = msg::load().unwrap();

    debug!("Received action: {:?}", action);

    match action {
        Action::AddCandidate(name) => {
            STATE.add_candidate(name.clone());

            msg::reply((), 0, 0);

            debug!("Added new candidate: {:?}", name);
        }

        Action::VoteForCandidate(name) => {
            STATE.vote_for_candidate(name.clone());

            msg::reply((), 0, 0);

            debug!("Voted for: {:?}", name);
        }
    }
}

// The function that returns a part of memory with a state
#[no_mangle]
pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let query: StateAction = msg::load().expect("failed to decode input argument");

    let encoded = match query {
        StateAction::All => StateReply::All(STATE.votes_received.clone()).encode(),

        StateAction::VotesFor(name) => {
            let votes_for_candidate = STATE
                .votes_received
                .get(&name)
                .expect("Can't find any candidate");

            StateReply::VotesFor(votes_for_candidate.clone()).encode()
        }
    };

    let result = gstd::macros::util::to_wasm_ptr(&encoded[..]);
    core::mem::forget(encoded);

    result
}
