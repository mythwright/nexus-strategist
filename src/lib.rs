use std::os::raw::c_char;
use std::sync::{OnceLock, RwLock};
use arcdps::evtc;
use nexus::{AddonFlags, event_subscribe, log, UpdateProvider};

nexus::export! {
    name: "Nexus Strategist",
    signature: -50604,
    load,
    unload,
    flags: AddonFlags::None,
    provider: UpdateProvider::GitHub,
    update_link: "https://github.com/mythwright/nexus-strategist",
}

static mut STRAT_MANAGER: OnceLock<StratManager> = OnceLock::new();

fn load() {
    unsafe { STRAT_MANAGER.set(StratManager::new()).unwrap()};

    event_subscribe!(unsafe "EV_ARCDPS_COMBATEVENT_LOCAL_RAW" => EvCombatData, callback)
        .revert_on_unload();

}

#[derive(Debug)]
#[repr(C)]
struct EvCombatData {
    pub ev: *const evtc::Event,
    pub src: *const evtc::Agent,
    pub dst: *const evtc::Agent,
    pub skill_name: *const c_char,
    pub id: u64,
    pub revision: u64,
}

fn callback(ev: Option<&EvCombatData>) {
    let evsrc = unsafe { ev.unwrap().src.to_owned().read() };
    log::log(
        log::LogLevel::Debug,
        "Nexus Strategist",
        format!("The event src is: {:#?}", evsrc),
    );


    unsafe {
        STRAT_MANAGER.get_mut().unwrap().add_event(evsrc);
    }
}

fn unload() {
    let _ = unsafe {STRAT_MANAGER.take()};
}

#[derive(Debug)]
struct StratManager {
    pub exchange_event: RwLock<Vec<evtc::Agent>>,
}

impl StratManager {
    pub fn new() -> StratManager {
        StratManager {
            exchange_event: RwLock::new(Vec::new())
        }
    }

    pub fn add_event(&mut self, ev: evtc::Agent) {
        self.exchange_event.get_mut()
            .unwrap()
            .push(ev)
        ;
        log::log(
            log::LogLevel::Debug,
            "Nexus Strategist",
            format!("The len of the queue: {:?}", self.exchange_event.read().unwrap().len()),
        );

    }
}