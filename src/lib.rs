#![feature(const_btree_new)]
#![no_std]

use codec::{Decode, Encode};
use gstd::{debug, msg, prelude::*};
use scale_info::TypeInfo;

gstd::metadata! {
    title: "Voting_App",
    handle:
        input: Action,
    state:
        output: BTreeMap<String, i32>,
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

    // Получить информацию по всем кандидатам
    pub fn get_candidates(&self) -> BTreeMap<String, i32> {
        self.votes_received.clone()
    }
}

// 3️⃣ Иницилизируем стейт
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

            debug!("Voted f: {:?}", name);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let votes: BTreeMap<String, i32> = STATE.votes_received.clone();
    let encoded = votes.encode();
    let result = gstd::macros::util::to_wasm_ptr(&encoded[..]);
    core::mem::forget(encoded);

    result
}
