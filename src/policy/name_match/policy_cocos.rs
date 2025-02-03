use crate::name_match_policy;
#[cfg(debug_assertions)]
use log::debug;

const TOP: [&str; 0] = [];
const ONLY6: [&str; 0] = [];
const ONLY7: [&str; 1] = ["GLThread"];
const MIDDLE: [&str; 0] = [];
const BACKEND: [&str; 0] = [];

name_match_policy!(start_task, &TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND);
