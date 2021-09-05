use std::{sync::Mutex};
use serde::{Deserialize, Serialize};
use data::types::CDDAKnowledgeGraph;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppStateRaw {
  pub knowledge_graph: CDDAKnowledgeGraph,
}

pub struct AppState(pub Mutex<AppStateRaw>); // need pub, otherwise will be "field `0` of struct `types::state::AppState` is private"
