use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct KeypairResponseData {
    pub pubkey: String,
    pub secret: String,
}

#[derive(Serialize)]
pub struct KeypairResponse {
    pub success: bool,
    pub data: Option<KeypairResponseData>,
    pub error: Option<String>,
}

#[derive(Deserialize)]
pub struct TokenCreateRequest {
    pub mintAuthority: String,
    pub mint: String,
    pub decimals: u8,
}

#[derive(Serialize)]
pub struct AccountMetaModel {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Serialize)]
pub struct TokenInstructionResponseData {
    pub program_id: String,
    pub accounts: Vec<AccountMetaModel>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct TokenInstructionResponse {
    pub success: bool,
    pub data: Option<TokenInstructionResponseData>,
    pub error: Option<String>,
}

#[derive(Deserialize)]
pub struct TokenMintRequest {
    pub mint: String,
    pub destination: String,
    pub authority: String,
    pub amount: u64,
}

#[derive(Deserialize)]
pub struct MessageSignRequest {
    pub message: String,
    pub secret: String,
}

#[derive(Serialize)]
pub struct MessageSignResponseData {
    pub signature: String,
    #[serde(rename = "pubkey")]
    pub pubkey: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct MessageSignResponse {
    pub success: bool,
    pub data: Option<MessageSignResponseData>,
    pub error: Option<String>,
}

#[derive(Deserialize)]
pub struct MessageVerifyRequest {
    pub message: String,
    pub signature: String,
    pub pubkey: String,
}

#[derive(Serialize)]
pub struct MessageVerifyResponseData {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}

#[derive(Serialize)]
pub struct MessageVerifyResponse {
    pub success: bool,
    pub data: Option<MessageVerifyResponseData>,
    pub error: Option<String>,
}

#[derive(Deserialize)]
pub struct SendSolRequest {
    pub from: String,
    pub to: String,
    pub lamports: u64,
}

#[derive(Serialize)]
pub struct SendSolResponseData {
    pub program_id: String,
    pub accounts: Vec<String>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct SendSolResponse {
    pub success: bool,
    pub data: Option<SendSolResponseData>,
    pub error: Option<String>,
}

#[derive(Deserialize)]
pub struct SendTokenRequest {
    pub destination: String,
    pub mint: String,
    pub owner: String,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct SendTokenAccountMeta {
    pub pubkey: String,
    pub isSigner: bool,
}

#[derive(Serialize)]
pub struct SendTokenResponseData {
    pub program_id: String,
    pub accounts: Vec<SendTokenAccountMeta>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct SendTokenResponse {
    pub success: bool,
    pub data: Option<SendTokenResponseData>,
    pub error: Option<String>,
} 