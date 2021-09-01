use data::types::CDDAKnowledgeGraph;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppState {
  knowledge_graph: CDDAKnowledgeGraph,
}
