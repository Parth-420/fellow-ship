use axum::{Json, http::StatusCode};
use serde_json::json;
use solana_sdk::signature::{Keypair, Signer};
use bs58;
use crate::models::{KeypairResponse, KeypairResponseData};
use axum::extract::Json as AxumJson;
use crate::models::{TokenCreateRequest, TokenInstructionResponse, TokenInstructionResponseData, AccountMetaModel, TokenMintRequest};
use solana_program::pubkey::Pubkey;
use spl_token::instruction::initialize_mint;
use spl_token::instruction::mint_to;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use crate::models::{MessageSignRequest, MessageSignResponse, MessageSignResponseData};
use solana_sdk::signature::Signature;
use crate::models::{MessageVerifyRequest, MessageVerifyResponse, MessageVerifyResponseData};
use crate::models::{SendSolRequest, SendSolResponse, SendSolResponseData};
use solana_sdk::system_instruction;
use crate::models::{SendTokenRequest, SendTokenResponse, SendTokenResponseData, SendTokenAccountMeta};
use spl_token::instruction::transfer as spl_transfer;
use std::str::FromStr;
use ed25519_dalek::{PublicKey as DalekPublicKey, Signature as DalekSignature, Verifier};

pub async fn keypair_handler() -> (StatusCode, Json<KeypairResponse>) {
    match Keypair::new() {
        keypair => {
            let pubkey = keypair.pubkey().to_string();
            let secret = bs58::encode(keypair.to_bytes()).into_string();
            let data = KeypairResponseData { pubkey, secret };
            (
                StatusCode::OK,
                Json(KeypairResponse {
                    success: true,
                    data: Some(data),
                    error: None,
                })
            )
        }
    }
}

pub async fn token_create_handler(
    AxumJson(req): AxumJson<serde_json::Value>,
) -> (StatusCode, AxumJson<TokenInstructionResponse>) {
    let mint = req.get("mint").and_then(|v| v.as_str());
    let mint_authority = req.get("mintAuthority").and_then(|v| v.as_str());
    let decimals = req.get("decimals").and_then(|v| v.as_u64());
    if mint.is_none() || mint_authority.is_none() || decimals.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            AxumJson(TokenInstructionResponse {
                success: false,
                data: None,
                error: Some("Missing required fields".to_string()),
            })
        );
    }
    // validate pubkeys -realisation 1
    if Pubkey::from_str(mint.unwrap()).is_err() {
        return (
            StatusCode::BAD_REQUEST,
            AxumJson(TokenInstructionResponse {
                success: false,
                data: None,
                error: Some("Invalid mint pubkey".to_string()),
            })
        );
    }
    if Pubkey::from_str(mint_authority.unwrap()).is_err() {
        return (
            StatusCode::BAD_REQUEST,
            AxumJson(TokenInstructionResponse {
                success: false,
                data: None,
                error: Some("Invalid mintAuthority pubkey".to_string()),
            })
        );
    }
    let req = TokenCreateRequest {
        mint: mint.unwrap().to_string(),
        mintAuthority: mint_authority.unwrap().to_string(),
        decimals: decimals.unwrap() as u8,
    };
    
    let ix = match initialize_mint(
        &spl_token::id(),
        &Pubkey::from_str(&req.mint).unwrap(),
        &Pubkey::from_str(&req.mintAuthority).unwrap(),
        None,
        req.decimals,
    ) {
        Ok(ix) => ix,
        Err(e) => return (
            StatusCode::OK,
            AxumJson(TokenInstructionResponse {
                success: false,
                data: None,
                error: Some(format!("Failed to create instruction: {}", e)),
            })
        ),
    };
    let accounts: Vec<AccountMetaModel> = ix.accounts.iter().map(|meta| AccountMetaModel {
        pubkey: meta.pubkey.to_string(),
        is_signer: meta.is_signer,
        is_writable: meta.is_writable,
    }).collect();
    let instruction_data = BASE64.encode(ix.data);
    let data = TokenInstructionResponseData {
        program_id: ix.program_id.to_string(),
        accounts,
        instruction_data,
    };
    (
        StatusCode::OK,
        AxumJson(TokenInstructionResponse {
            success: true,
            data: Some(data),
            error: None,
        })
    )
}

pub async fn token_mint_handler(
    AxumJson(req): AxumJson<serde_json::Value>,
) -> (StatusCode, AxumJson<TokenInstructionResponse>) {
    let mint = req.get("mint").and_then(|v| v.as_str());
    let destination = req.get("destination").and_then(|v| v.as_str());
    let authority = req.get("authority").and_then(|v| v.as_str());
    let amount = req.get("amount").and_then(|v| v.as_u64());
    if mint.is_none() || destination.is_none() || authority.is_none() || amount.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            AxumJson(TokenInstructionResponse {
                success: false,
                data: None,
                error: Some("Missing required fields".to_string()),
            })
        );
    }
    if Pubkey::from_str(mint.unwrap()).is_err() {
        return (
            StatusCode::BAD_REQUEST,
            AxumJson(TokenInstructionResponse {
                success: false,
                data: None,
                error: Some("Invalid mint pubkey".to_string()),
            })
        );
    }
    if Pubkey::from_str(destination.unwrap()).is_err() {
        return (
            StatusCode::BAD_REQUEST,
            AxumJson(TokenInstructionResponse {
                success: false,
                data: None,
                error: Some("Invalid destination pubkey".to_string()),
            })
        );
    }
    if Pubkey::from_str(authority.unwrap()).is_err() {
        return (
            StatusCode::BAD_REQUEST,
            AxumJson(TokenInstructionResponse {
                success: false,
                data: None,
                error: Some("Invalid authority pubkey".to_string()),
            })
        );
    }
    let req = TokenMintRequest {
        mint: mint.unwrap().to_string(),
        destination: destination.unwrap().to_string(),
        authority: authority.unwrap().to_string(),
        amount: amount.unwrap(),
    };
    let ix = match mint_to(
        &spl_token::id(),
        &Pubkey::from_str(&req.mint).unwrap(),
        &Pubkey::from_str(&req.destination).unwrap(),
        &Pubkey::from_str(&req.authority).unwrap(),
        &[],
        req.amount,
    ) {
        Ok(ix) => ix,
        Err(e) => return (
            StatusCode::OK,
            AxumJson(TokenInstructionResponse {
                success: false,
                data: None,
                error: Some(format!("Failed to create instruction: {}", e)),
            })
        ),
    };
    let accounts: Vec<AccountMetaModel> = ix.accounts.iter().map(|meta| AccountMetaModel {
        pubkey: meta.pubkey.to_string(),
        is_signer: meta.is_signer,
        is_writable: meta.is_writable,
    }).collect();
    let instruction_data = BASE64.encode(ix.data);
    let data = TokenInstructionResponseData {
        program_id: ix.program_id.to_string(),
        accounts,
        instruction_data,
    };
    (
        StatusCode::OK,
        AxumJson(TokenInstructionResponse {
            success: true,
            data: Some(data),
            error: None,
        })
    )
}

pub async fn message_sign_handler(
    AxumJson(req): AxumJson<serde_json::Value>,
) -> (StatusCode, AxumJson<MessageSignResponse>) {
    let message = req.get("message").and_then(|v| v.as_str());
    let secret = req.get("secret").and_then(|v| v.as_str());
    if message.is_none() || secret.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            AxumJson(MessageSignResponse {
                success: false,
                data: None,
                error: Some("Missing required fields".to_string()),
            })
        );
    }
    // validate base58 secret key realisation 1
    let secret_bytes = match bs58::decode(secret.unwrap()).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return (
            StatusCode::BAD_REQUEST,
            AxumJson(MessageSignResponse {
                success: false,
                data: None,
                error: Some("Invalid base58 secret key".to_string()),
            })
        ),
    };
    let keypair = match Keypair::from_bytes(&secret_bytes) {
        Ok(kp) => kp,
        Err(_) => return (
            StatusCode::BAD_REQUEST,
            AxumJson(MessageSignResponse {
                success: false,
                data: None,
                error: Some("Invalid secret key bytes".to_string()),
            })
        ),
    };
    let req = MessageSignRequest {
        message: message.unwrap().to_string(),
        secret: secret.unwrap().to_string(),
    };
    let message_bytes = req.message.as_bytes();
    let signature = keypair.sign_message(message_bytes);
    let signature_b64 = BASE64.encode(signature.as_ref());
    let data = MessageSignResponseData {
        signature: signature_b64,
        public_key: keypair.pubkey().to_string(),
        message: req.message,
    };
    (
        StatusCode::OK,
        AxumJson(MessageSignResponse {
            success: true,
            data: Some(data),
            error: None,
        })
    )
}

pub async fn message_verify_handler(
    AxumJson(req): AxumJson<serde_json::Value>,
) -> (StatusCode, AxumJson<MessageVerifyResponse>) {
    let message = req.get("message").and_then(|v| v.as_str());
    let signature_str = req.get("signature").and_then(|v| v.as_str());
    let pubkey = req.get("pubkey").and_then(|v| v.as_str());
    if message.is_none() || signature_str.is_none() || pubkey.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            AxumJson(MessageVerifyResponse {
                success: false,
                data: None,
                error: Some("Missing required fields".to_string()),
            })
        );
    }
    // validate pubkey realisation 1
    let pubkey_val = match Pubkey::from_str(pubkey.unwrap()) {
        Ok(pk) => pk,
        Err(_) => return (
            StatusCode::BAD_REQUEST,
            AxumJson(MessageVerifyResponse {
                success: false,
                data: None,
                error: Some("Invalid pubkey".to_string()),
            })
        ),
    };
    // validate base64 signature realisation 1
    let signature_bytes = match BASE64.decode(signature_str.unwrap()) {
        Ok(bytes) => bytes,
        Err(_) => return (
            StatusCode::BAD_REQUEST,
            AxumJson(MessageVerifyResponse {
                success: false,
                data: None,
                error: Some("Invalid base64 signature".to_string()),
            })
        ),
    };
    if signature_bytes.len() != 64 {
        return (
            StatusCode::BAD_REQUEST,
            AxumJson(MessageVerifyResponse {
                success: false,
                data: None,
                error: Some("Signature must be 64 bytes".to_string()),
            })
        );
    }
    let dalek_signature = match DalekSignature::from_bytes(&signature_bytes) {
        Ok(sig) => sig,
        Err(_) => return (
            StatusCode::BAD_REQUEST,
            AxumJson(MessageVerifyResponse {
                success: false,
                data: None,
                error: Some("Invalid signature bytes".to_string()),
            })
        ),
    };
    let pubkey_bytes = pubkey_val.to_bytes();
    let dalek_pubkey = match DalekPublicKey::from_bytes(&pubkey_bytes) {
        Ok(pk) => pk,
        Err(_) => return (
            StatusCode::BAD_REQUEST,
            AxumJson(MessageVerifyResponse {
                success: false,
                data: None,
                error: Some("Invalid public key bytes for ed25519".to_string()),
            })
        ),
    };
    let req = MessageVerifyRequest {
        message: message.unwrap().to_string(),
        signature: signature_str.unwrap().to_string(),
        pubkey: pubkey.unwrap().to_string(),
    };
    let valid = dalek_pubkey.verify(req.message.as_bytes(), &dalek_signature).is_ok();
    let data = MessageVerifyResponseData {
        valid,
        message: req.message,
        pubkey: req.pubkey,
    };
    (
        StatusCode::OK,
        AxumJson(MessageVerifyResponse {
            success: true,
            data: Some(data),
            error: None,
        })
    )
}

pub async fn send_sol_handler(
    AxumJson(req): AxumJson<serde_json::Value>,
) -> (StatusCode, AxumJson<SendSolResponse>) {
    let from = req.get("from").and_then(|v| v.as_str());
    let to = req.get("to").and_then(|v| v.as_str());
    let lamports = req.get("lamports").and_then(|v| v.as_u64());
    if from.is_none() || to.is_none() || lamports.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            AxumJson(SendSolResponse {
                success: false,
                data: None,
                error: Some("Missing required fields".to_string()),
            })
        );
    }
    if lamports.unwrap() == 0 {
        return (
            StatusCode::BAD_REQUEST,
            AxumJson(SendSolResponse {
                success: false,
                data: None,
                error: Some("Lamports must be greater than zero".to_string()),
            })
        );
    }
    if Pubkey::from_str(from.unwrap()).is_err() {
        return (
            StatusCode::BAD_REQUEST,
            AxumJson(SendSolResponse {
                success: false,
                data: None,
                error: Some("Invalid from pubkey".to_string()),
            })
        );
    }
    if Pubkey::from_str(to.unwrap()).is_err() {
        return (
            StatusCode::BAD_REQUEST,
            AxumJson(SendSolResponse {
                success: false,
                data: None,
                error: Some("Invalid to pubkey".to_string()),
            })
        );
    }
    let req = SendSolRequest {
        from: from.unwrap().to_string(),
        to: to.unwrap().to_string(),
        lamports: lamports.unwrap(),
    };
    let ix = system_instruction::transfer(&Pubkey::from_str(&req.from).unwrap(), &Pubkey::from_str(&req.to).unwrap(), req.lamports);
    let accounts = ix.accounts.iter().map(|meta| meta.pubkey.to_string()).collect();
    let instruction_data = BASE64.encode(ix.data);
    let data = SendSolResponseData {
        program_id: ix.program_id.to_string(),
        accounts,
        instruction_data,
    };
    (
        StatusCode::OK,
        AxumJson(SendSolResponse {
            success: true,
            data: Some(data),
            error: None,
        })
    )
}

pub async fn send_token_handler(
    AxumJson(req): AxumJson<serde_json::Value>,
) -> (StatusCode, AxumJson<SendTokenResponse>) {
    let destination = req.get("destination").and_then(|v| v.as_str());
    let mint = req.get("mint").and_then(|v| v.as_str());
    let owner = req.get("owner").and_then(|v| v.as_str());
    let amount = req.get("amount").and_then(|v| v.as_u64());
    if destination.is_none() || mint.is_none() || owner.is_none() || amount.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            AxumJson(SendTokenResponse {
                success: false,
                data: None,
                error: Some("Missing required fields".to_string()),
            })
        );
    }
    if Pubkey::from_str(destination.unwrap()).is_err() {
        return (
            StatusCode::BAD_REQUEST,
            AxumJson(SendTokenResponse {
                success: false,
                data: None,
                error: Some("Invalid destination pubkey".to_string()),
            })
        );
    }
    if Pubkey::from_str(mint.unwrap()).is_err() {
        return (
            StatusCode::BAD_REQUEST,
            AxumJson(SendTokenResponse {
                success: false,
                data: None,
                error: Some("Invalid mint pubkey".to_string()),
            })
        );
    }
    if Pubkey::from_str(owner.unwrap()).is_err() {
        return (
            StatusCode::BAD_REQUEST,
            AxumJson(SendTokenResponse {
                success: false,
                data: None,
                error: Some("Invalid owner pubkey".to_string()),
            })
        );
    }
    let req = SendTokenRequest {
        destination: destination.unwrap().to_string(),
        mint: mint.unwrap().to_string(),
        owner: owner.unwrap().to_string(),
        amount: amount.unwrap(),
    };
   
    let ix = match spl_transfer(
        &spl_token::id(),
        &Pubkey::from_str(&req.owner).unwrap(), 
        &Pubkey::from_str(&req.destination).unwrap(), 
        &Pubkey::from_str(&req.owner).unwrap(), 
        &[],
        req.amount,
    ) {
        Ok(ix) => ix,
        Err(e) => return (
            StatusCode::OK,
            AxumJson(SendTokenResponse {
                success: false,
                data: None,
                error: Some(format!("Failed to create instruction: {}", e)),
            })
        ),
    };
    let accounts: Vec<SendTokenAccountMeta> = ix.accounts.iter().map(|meta| SendTokenAccountMeta {
        pubkey: meta.pubkey.to_string(),
        isSigner: meta.is_signer,
    }).collect();
    let instruction_data = BASE64.encode(ix.data);
    let data = SendTokenResponseData {
        program_id: ix.program_id.to_string(),
        accounts,
        instruction_data,
    };
    (
        StatusCode::OK,
        AxumJson(SendTokenResponse {
            success: true,
            data: Some(data),
            error: None,
        })
    )
} 