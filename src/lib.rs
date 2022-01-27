#![feature(const_btree_new)]
#![no_std]

// 1️⃣ External packages (crates) and internal modules import
use codec::{Decode, Encode};
use gstd::{debug, msg, prelude::*};
use scale_info::TypeInfo;

gstd::metadata! {
    title: "Voting App",
    handle:
        input: Action,
    state:
        input: Option<String>,
        output: BTreeMap<String, u32>,
}

#[derive(Debug, TypeInfo, Decode)]
pub enum Action {
    AddCandidate(String),
    VoteForCandidate(String),
}

#[derive(Clone)]
pub struct State {
    votes_received: BTreeMap<String, i32>,
}

impl State {
    // Создать стейт
    pub const fn new() -> Self {
        Self {
            votes_received: BTreeMap::new(),
        }
    }

    // Добавить кандидата
    pub fn add_candidate(&mut self, candidate: String) {
        self.votes_received.insert(candidate, 0);
    }

    // Проголосовать за кандидата по имени
    pub fn vote_for_candidate(&mut self, name: String) {
        let counter = self.votes_received.entry(name).or_insert(0);
        *counter += 1;
    }

    // Получить голоса по имени кандидата
    pub fn get_total_votes_for(self, name: String) -> Option<i32> {
        self.votes_received.get(&name).cloned()
    }
}

// Иницилизируем стейт
static mut STATE: State = State::new();

#[no_mangle]
pub unsafe extern "C" fn init() {}

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

#[no_mangle]
pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let candidate: Option<String> = msg::load().expect("failed to decode input argument");

    let encoded = match candidate {
        None => STATE.votes_received.clone().encode(),
        Some(name) => {
            let votes_for_candidate = STATE
                .votes_received
                .get(&name)
                .expect("can't find any candidate");

            votes_for_candidate.encode()
        }
    };

    let result = gstd::macros::util::to_wasm_ptr(&encoded[..]);
    core::mem::forget(encoded);

    result
}
