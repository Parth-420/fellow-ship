# Solana Fellowship API

A Rust-based REST API for Solana blockchain operations, built with Axum framework. This API provides endpoints for keypair generation, token operations, message signing/verification, and SOL/SPL token transfers.

## Features

- **Keypair Generation**: Generate new Solana keypairs
- **Token Operations**: Create and mint SPL tokens
- **Message Signing**: Sign messages with Ed25519 signatures
- **Message Verification**: Verify message signatures
- **SOL Transfers**: Create SOL transfer instructions
- **SPL Token Transfers**: Create SPL token transfer instructions

## Tech Stack

- **Backend**: Rust with Axum framework
- **Blockchain**: Solana SDK and SPL Token
- **Cryptography**: Ed25519 for message signing
- **Serialization**: Serde for JSON handling
- **Testing**: Jest with Node.js

## Prerequisites

- Rust (latest stable version)
- Node.js (v16 or higher)
- npm or yarn

## Installation

### Backend Setup

1. Clone the repository:
```bash
git clone <repository-url>
cd fellowship
```

2. Install Rust dependencies:
```bash
cargo build
```

3. Run the server:
```bash
cargo run
```


### Testing Setup

1. Navigate to the tests directory:
```bash
cd tests
```

2. Install Node.js dependencies:
```bash
npm install
```

3. Run the tests:
```bash
npx jest test.js
```

## API Endpoints

### Keypair Operations

#### POST /keypair
Generate a new Solana keypair.

**Request Body**: None

**Response**: Returns a new keypair with public key and secret key (base58 encoded).

### Token Operations

#### POST /token/create
Create a new SPL token mint instruction.

**Request Body**:
```json
{
  "mint": "string",           // Mint public key
  "mintAuthority": "string",  // Mint authority public key
  "decimals": number          // Token decimals (0-9)
}
```

#### POST /token/mint
Create a mint-to instruction for SPL tokens.

**Request Body**:
```json
{
  "mint": "string",        // Mint public key
  "destination": "string", // Destination public key
  "authority": "string",   // Authority public key
  "amount": number         // Amount to mint
}
```

### Message Operations

#### POST /message/sign
Sign a message with a private key.

**Request Body**:
```json
{
  "message": "string",  // Message to sign
  "secret": "string"    // Private key (base58 encoded)
}
```

#### POST /message/verify
Verify a message signature.

**Request Body**:
```json
{
  "message": "string",   // Original message
  "signature": "string", // Signature (base58 encoded)
  "pubkey": "string"     // Public key
}
```

### Transfer Operations

#### POST /send/sol
Create a SOL transfer instruction.

**Request Body**:
```json
{
  "from": "string",     // Sender public key
  "to": "string",       // Recipient public key
  "lamports": number    // Amount in lamports
}
```

#### POST /send/token
Create an SPL token transfer instruction.

**Request Body**:
```json
{
  "destination": "string", // Destination public key
  "mint": "string",        // Token mint public key
  "owner": "string",       // Token owner public key
  "amount": number         // Amount to transfer
}
```

## Testing

### Running Tests

1. **Start the server** (in one terminal):
```bash
cargo run
```

2. **Run tests** (in another terminal):
```bash
cd tests
npx jest test.js
```

### Test Environment

The tests use the following environment variables:
- `HTTP_URL`: API server URL (defaults to `https://fellow-ship.onrender.com`)

To test against a local server:
```bash
HTTP_URL=http://localhost:3000 npx jest test.js
```

### Test Coverage

The test suite covers:
- ✅ Keypair generation and validation
- ✅ Token creation and minting
- ✅ Message signing and verification
- ✅ SOL transfer instruction creation
- ✅ SPL token transfer instruction creation
- ✅ Error handling for invalid inputs
- ✅ Input validation and edge cases

## Project Structure

```
fellowship/
├── src/
│   ├── main.rs          # Application entry point
│   ├── handlers.rs      # Request handlers
│   ├── models.rs        # Data structures
│   └── routes.rs        # Route definitions
├── tests/
│   ├── test.js          # Jest test suite
│   ├── package.json     # Node.js dependencies
│   └── jest.config.js   # Jest configuration
├── Cargo.toml           # Rust dependencies
└── README.md           # This file
```

## Dependencies

### Rust Dependencies (Cargo.toml)
- `axum`: Web framework
- `solana-sdk`: Solana blockchain SDK
- `spl-token`: SPL token operations
- `spl-associated-token-account`: Associated token account utilities
- `ed25519-dalek`: Ed25519 cryptography
- `bs58`: Base58 encoding/decoding
- `serde`: Serialization/deserialization

### Node.js Dependencies (tests/package.json)
- `jest`: Testing framework
- `axios`: HTTP client
- `@solana/web3.js`: Solana JavaScript SDK
- `@solana/spl-token`: SPL token JavaScript SDK
- `bs58`: Base58 encoding/decoding
- `tweetnacl`: Cryptography library

## Development

### Adding New Endpoints

1. Define the request/response models in `src/models.rs`
2. Implement the handler in `src/handlers.rs`
3. Add the route in `src/routes.rs`
4. Write tests in `tests/test.js`

### Code Style

- Follow Rust conventions and use `cargo fmt` for formatting
- Use meaningful variable names and add comments for complex logic
- Ensure all endpoints have proper error handling
- Write comprehensive tests for new functionality

## Deployment

The API can be deployed to any platform that supports Rust applications:

- **Render**: Configure as a Rust web service
- **Railway**: Deploy with automatic Rust detection
- **Heroku**: Use the Rust buildpack
- **Docker**: Create a Dockerfile for containerized deployment

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

