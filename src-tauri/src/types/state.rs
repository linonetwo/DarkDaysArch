use std::{sync::Mutex};
use data::types::CDDAKnowledgeGraph;

#[derive(Debug)]
pub struct AppStateRaw {
  pub knowledge_graph: CDDAKnowledgeGraph,
}

pub struct AppState(pub Mutex<AppStateRaw>); // need pub, otherwise will be "field `0` of struct `types::state::AppState` is private"
