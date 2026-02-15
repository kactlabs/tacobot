//! PKCE (Proof Key for Public Clients) implementation

use serde::{Deserialize, Serialize};

/// PKCE challenge and verifier pair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PkceChallenge {
    pub code_verifier: String,
    pub code_challenge: String,
}
