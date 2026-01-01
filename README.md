# 🦀 Rust Programming & Cryptography - Complete Learning Guide

> A comprehensive, beginner-friendly guide to learning Rust programming and implementing cryptographic algorithms from scratch.

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Learning](https://img.shields.io/badge/Level-Beginner%20to%20Intermediate-green.svg)]()

## 📚 Table of Contents

- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [Repository Structure](#repository-structure)
- [Getting Started](#getting-started)
- [Learning Path](#learning-path)
- [Rust Fundamentals](#rust-fundamentals)
- [Cryptography Implementations](#cryptography-implementations)
- [Setup Instructions](#setup-instructions)
- [Running the Code](#running-the-code)
- [Exercises](#exercises)
- [Resources](#resources)
- [Contributing](#contributing)
- [License](#license)

## 🎯 Overview

This repository contains a complete learning curriculum for:

1. **Rust Programming Language** - From absolute beginner to intermediate level
2. **Cryptography Fundamentals** - Understanding and implementing crypto algorithms
3. **Practical Applications** - Building real-world security tools

All code examples are:
- ✅ Fully functional and tested
- ✅ Heavily commented for clarity
- ✅ Progressive in difficulty
- ✅ Production-quality code patterns (for learning, not production crypto!)

## 📋 Prerequisites

**Required:**
- Basic programming concepts (variables, functions, loops)
- Willingness to learn and experiment
- A computer with internet connection

**Helpful but not required:**
- Understanding of basic data structures
- Familiarity with any programming language
- Interest in security and cryptography

## 📁 Repository Structure

```
rust-crypto-learning/
│
├── README.md                          # This file
├── Cargo.toml                         # Rust project configuration
│
├── 01-rust-fundamentals/
│   ├── 01-variables-mutability.rs     # Variables, mut, shadowing
│   ├── 02-data-types.rs               # All Rust data types
│   ├── 03-functions.rs                # Functions, closures, recursion
│   ├── 04-control-flow.rs             # if, loops, match
│   ├── 05-ownership.rs                # THE most important concept
│   ├── 06-structs.rs                  # Custom types and methods
│   ├── 07-enums.rs                    # Enums, Option, Result
│   └── 08-collections.rs              # Vec, HashMap, HashSet
│
├── 02-cryptography-basics/
│   ├── 01-caesar-cipher.rs            # Simple substitution cipher
│   ├── 02-xor-cipher.rs               # XOR operations
│   ├── 03-sha256-hashing.rs           # SHA-256 hash function
│   ├── 04-aes-encryption.rs           # AES-256-GCM symmetric encryption
│   ├── 05-rsa-asymmetric.rs           # RSA public key cryptography
│   ├── 06-digital-signatures.rs       # Ed25519 signatures
│   ├── 07-password-hashing.rs         # Argon2 secure password hashing
│   └── 08-simple-blockchain.rs        # Basic blockchain implementation
│
├── 03-practical-projects/
│   ├── file-encryptor/                # Encrypt/decrypt files
│   ├── password-manager/              # Secure password storage
│   ├── secure-chat/                   # End-to-end encrypted messaging
│   └── mini-blockchain/               # Extended blockchain with mining
│
├── exercises/
│   ├── rust-exercises.md              # Practice problems
│   └── crypto-challenges.md           # Cryptography challenges
│
└── docs/
    ├── LEARNING_PATH.md               # Detailed learning roadmap
    ├── RUST_CHEATSHEET.md             # Quick reference
    ├── CRYPTO_CONCEPTS.md             # Cryptography theory
    └── BEST_PRACTICES.md              # Security and code quality
```

## 🚀 Getting Started

### 1. Install Rust

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

### 2. Clone This Repository

```bash
git clone https://github.com/yourusername/rust-crypto-learning.git
cd rust-crypto-learning
```

### 3. Run Your First Example

```bash
# Run the variables example
cargo run --bin variables-mutability

# Or navigate to a specific file
cd 01-rust-fundamentals
rustc 01-variables-mutability.rs
./01-variables-mutability
```

## 📖 Learning Path

Follow this sequence for optimal learning:

### Phase 1: Rust Foundations (Week 1-2)

1. **Variables & Mutability** (Day 1)
   - Understand immutability by default
   - Learn `mut` keyword
   - Practice shadowing

2. **Data Types** (Day 2-3)
   - Scalar types: integers, floats, booleans, chars
   - Compound types: tuples, arrays
   - String vs &str

3. **Functions** (Day 4)
   - Function syntax
   - Parameters and return values
   - Closures

4. **Control Flow** (Day 5)
   - if/else expressions
   - Loops: loop, while, for
   - Pattern matching with match

5. **Ownership** (Day 6-8) ⭐ **CRITICAL**
   - Move semantics
   - Borrowing and references
   - Lifetimes basics

6. **Structs & Enums** (Day 9-10)
   - Custom data types
   - Methods and associated functions
   - Option and Result

7. **Collections** (Day 11-12)
   - Vectors
   - HashMaps
   - Strings in depth

### Phase 2: Cryptography Basics (Week 3-4)

1. **Caesar Cipher** (Day 13)
   - Understanding substitution
   - Brute force attacks

2. **XOR Cipher** (Day 14)
   - Bitwise operations
   - One-time pad concept

3. **Hash Functions** (Day 15-16)
   - SHA-256 implementation
   - Hash properties
   - Applications

4. **Symmetric Encryption** (Day 17-18)
   - AES-256-GCM
   - Encryption modes
   - Key management

5. **Asymmetric Encryption** (Day 19-20)
   - RSA algorithm
   - Public/private keys
   - Key exchange

6. **Digital Signatures** (Day 21-22)
   - Ed25519 signatures
   - Authentication
   - Non-repudiation

7. **Password Security** (Day 23-24)
   - Argon2 hashing
   - Salt and pepper
   - Rainbow tables

8. **Blockchain Basics** (Day 25-28)
   - Proof of work
   - Chain validation
   - Mining simulation

### Phase 3: Practical Projects (Week 5+)

Choose projects based on interest:
- File encryption tool
- Password manager
- Secure messaging app
- Cryptocurrency implementation

## 🦀 Rust Fundamentals

### 01. Variables & Mutability

**Key Concepts:**
- Immutable by default
- `mut` for mutability
- Shadowing vs reassignment
- Constants with `const`

**Example:**
```rust
let x = 5;           // Immutable
let mut y = 10;      // Mutable
y = 15;              // ✅ OK

let z = 5;
let z = z + 1;       // Shadowing - creates new variable
```

**Run:** `cargo run --bin variables-mutability`

### 02. Data Types

**Key Concepts:**
- Scalar: i32, f64, bool, char
- Compound: tuples, arrays
- Type inference
- Type annotations

**Example:**
```rust
let integer: i32 = 42;
let float: f64 = 3.14;
let tuple: (i32, f64, char) = (500, 6.4, 'A');
let array: [i32; 5] = [1, 2, 3, 4, 5];
```

**Run:** `cargo run --bin data-types`

### 03. Functions

**Key Concepts:**
- Function syntax
- Parameters (must have types)
- Return values
- Expressions vs statements

**Example:**
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // Expression - returned
}

fn main() {
    let result = add(5, 3);
    println!("Sum: {}", result);
}
```

**Run:** `cargo run --bin functions`

### 04. Control Flow

**Key Concepts:**
- if/else as expressions
- loop, while, for
- Pattern matching with match
- if let and while let

**Example:**
```rust
// match is exhaustive
match value {
    1 => println!("One"),
    2 => println!("Two"),
    _ => println!("Other"),
}

// for loop
for i in 1..5 {
    println!("{}", i);
}
```

**Run:** `cargo run --bin control-flow`

### 05. Ownership ⭐

**Key Concepts (CRITICAL):**
- Each value has ONE owner
- Owner goes out of scope = value dropped
- Move semantics
- Borrowing with &
- Mutable references with &mut

**Example:**
```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 MOVED to s2

// println!("{}", s1);  // ❌ ERROR!

// Borrowing instead
let s3 = String::from("hello");
let len = calculate_length(&s3);  // Borrow
println!("{}", s3);  // ✅ Still valid
```

**Run:** `cargo run --bin ownership`

### 06. Structs

**Key Concepts:**
- Custom data types
- Methods with `impl`
- Associated functions
- Tuple structs

**Example:**
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

**Run:** `cargo run --bin structs`

### 07. Enums & Pattern Matching

**Key Concepts:**
- Enums with data
- Option<T> for nullable values
- Result<T, E> for error handling
- Exhaustive matching

**Example:**
```rust
enum Option<T> {
    Some(T),
    None,
}

match some_option {
    Some(value) => println!("Has: {}", value),
    None => println!("Nothing"),
}
```

**Run:** `cargo run --bin enums`

### 08. Collections

**Key Concepts:**
- Vec<T> - growable arrays
- HashMap<K, V> - key-value pairs
- HashSet<T> - unique values
- String - UTF-8 text

**Example:**
```rust
let mut v = Vec::new();
v.push(1);
v.push(2);

let mut map = HashMap::new();
map.insert("key", "value");
```

**Run:** `cargo run --bin collections`

## 🔐 Cryptography Implementations

### 01. Caesar Cipher

**Concepts:** Substitution cipher, shift cipher, frequency analysis

**Security:** ❌ Very weak - only 26 possible keys

**Use Case:** Learning basic cryptography concepts

```rust
fn encrypt(text: &str, shift: u8) -> String {
    // Shift each letter by 'shift' positions
}
```

**Run:** `cargo run --bin caesar-cipher`

### 02. XOR Cipher

**Concepts:** Bitwise XOR, one-time pad, key reuse attacks

**Security:** ⚠️ Weak if key reused, perfect if key is truly random and same length

**Use Case:** Understanding symmetric encryption basics

```rust
fn xor_encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .zip(key.iter().cycle())
        .map(|(d, k)| d ^ k)
        .collect()
}
```

**Run:** `cargo run --bin xor-cipher`

### 03. SHA-256 Hashing

**Concepts:** One-way functions, collision resistance, integrity checking

**Security:** ✅ Cryptographically secure hash function

**Use Case:** Password verification, file integrity, digital signatures

```rust
use sha2::{Sha256, Digest};

let hash = Sha256::digest(b"hello world");
println!("{:x}", hash);
```

**Run:** `cargo run --bin sha256-hashing`

### 04. AES-256 Encryption

**Concepts:** Symmetric encryption, GCM mode, authenticated encryption

**Security:** ✅ Industry standard, military-grade encryption

**Use Case:** Encrypting data, secure storage, VPNs

```rust
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};

let cipher = Aes256Gcm::new(&key);
let ciphertext = cipher.encrypt(nonce, plaintext)?;
```

**Run:** `cargo run --bin aes-encryption`

### 05. RSA Encryption

**Concepts:** Asymmetric encryption, public/private keys, key exchange

**Security:** ✅ Secure with sufficient key size (2048+ bits)

**Use Case:** Secure key exchange, digital signatures, SSL/TLS

```rust
let private_key = RsaPrivateKey::new(&mut rng, 2048)?;
let public_key = RsaPublicKey::from(&private_key);
```

**Run:** `cargo run --bin rsa-asymmetric`

### 06. Digital Signatures

**Concepts:** Authentication, non-repudiation, integrity

**Security:** ✅ Ed25519 - fast and secure

**Use Case:** Software signing, blockchain, email (PGP)

```rust
let signing_key = SigningKey::generate(&mut rng);
let signature = signing_key.sign(message);
```

**Run:** `cargo run --bin digital-signatures`

### 07. Password Hashing

**Concepts:** Slow hashing, salting, memory-hard functions

**Security:** ✅ Argon2 - winner of password hashing competition

**Use Case:** User authentication, credential storage

```rust
let salt = SaltString::generate(&mut OsRng);
let hash = Argon2::default()
    .hash_password(password, &salt)?;
```

**Run:** `cargo run --bin password-hashing`

### 08. Simple Blockchain

**Concepts:** Proof of work, chain validation, distributed ledger

**Security:** ✅ Secure with sufficient difficulty

**Use Case:** Cryptocurrencies, supply chain, voting systems

```rust
struct Block {
    index: u64,
    timestamp: i64,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
}
```

**Run:** `cargo run --bin simple-blockchain`

## 🛠️ Setup Instructions

### Dependencies

Add to your `Cargo.toml`:

```toml
[dependencies]
# Hashing
sha2 = "0.10"
blake3 = "1.5"

# Symmetric Encryption
aes-gcm = "0.10"
chacha20poly1305 = "0.10"

# Asymmetric Encryption
rsa = "0.9"
ed25519-dalek = "2.0"

# Password Hashing
argon2 = "0.5"

# Utilities
rand = "0.8"
hex = "0.4"
base64 = "0.21"
chrono = "0.4"
```

### Building

```bash
# Check for errors
cargo check

# Build debug version
cargo build

# Build optimized version
cargo build --release

# Run specific example
cargo run --bin <example-name>

# Run all tests
cargo test

# Generate documentation
cargo doc --open
```

## 🏃 Running the Code

### Run Individual Examples

```bash
# Rust fundamentals
cargo run --bin variables-mutability
cargo run --bin ownership
cargo run --bin structs

# Cryptography
cargo run --bin caesar-cipher
cargo run --bin aes-encryption
cargo run --bin simple-blockchain
```

### Run All Examples

```bash
# Create a script run-all.sh
for file in 01-rust-fundamentals/*.rs; do
    echo "Running $(basename $file)..."
    rustc $file && ./$(basename $file .rs)
done
```

### Interactive Learning

1. Read the code comments carefully
2. Run the example
3. Modify the code
4. Uncomment error examples to see what breaks
5. Try the exercises at the bottom

## 📝 Exercises

### Rust Exercises

**Beginner:**
1. Create a temperature converter (Celsius ↔ Fahrenheit)
2. Implement FizzBuzz using match
3. Build a simple calculator
4. Create a struct for a Book with methods

**Intermediate:**
5. Implement a stack data structure
6. Build a simple todo list manager
7. Create a guessing game
8. Parse and validate user input

**Advanced:**
9. Implement a generic LinkedList
10. Build a simple expression evaluator

### Cryptography Exercises

**Beginner:**
1. Break Caesar cipher with frequency analysis
2. Implement Vigenère cipher
3. Create a hash-based file integrity checker
4. Build a password strength validator

**Intermediate:**
5. Implement hybrid encryption (RSA + AES)
6. Create a file encryption/decryption tool
7. Build a secure password generator
8. Implement HMAC for message authentication

**Advanced:**
9. Build a simple PKI (Public Key Infrastructure)
10. Create a basic cryptocurrency with wallet
11. Implement zero-knowledge proof concept
12. Build end-to-end encrypted chat

## 📚 Resources

### Official Rust Resources

- [The Rust Book](https://doc.rust-lang.org/book/) - Official guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by doing
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive exercises
- [Rust Playground](https://play.rust-lang.org/) - Online editor

### Cryptography Resources

- [Crypto101](https://www.crypto101.io/) - Free cryptography book
- [Cryptopals Challenges](https://cryptopals.com/) - Hands-on crypto challenges
- [Practical Cryptography](http://practicalcryptography.com/) - Algorithms explained
- [Serious Cryptography](https://nostarch.com/seriouscrypto) - By JP Aumasson

### Rust Crypto Libraries

- [RustCrypto](https://github.com/RustCrypto) - Pure Rust crypto implementations
- [Ring](https://github.com/briansmith/ring) - Safe, fast crypto using BoringSSL
- [Sodiumoxide](https://github.com/sodiumoxide/sodiumoxide) - Bindings to libsodium

### Communities

- [r/rust](https://reddit.com/r/rust) - Rust subreddit
- [Rust Discord](https://discord.gg/rust-lang) - Official Discord server
- [Rust Users Forum](https://users.rust-lang.org/) - Discussion forum
- [This Week in Rust](https://this-week-in-rust.org/) - Weekly newsletter

## 🤝 Contributing

Contributions are welcome! Here's how:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/improvement`)
3. Make your changes
4. Add tests if applicable
5. Commit with clear messages
6. Push to your fork
7. Open a Pull Request

**Contribution Ideas:**
- Add more examples
- Improve documentation
- Fix bugs or typos
- Add exercises
- Create practical projects
- Translate to other languages

## ⚠️ Important Notes

### Security Warning

**DO NOT use these implementations in production!**

These examples are for educational purposes only. For production use:
- Use well-tested, audited libraries
- Follow security best practices
- Get security review from experts
- Stay updated on vulnerabilities

### Learning Tips

1. **Type everything out** - Don't copy-paste
2. **Break things intentionally** - Learn from errors
3. **Experiment constantly** - Modify the code
4. **Read error messages** - Rust's compiler teaches you
5. **Ask questions** - Use forums and Discord
6. **Build projects** - Apply what you learn
7. **Review regularly** - Repetition solidifies knowledge

### Common Pitfalls

- Fighting the borrow checker (it's teaching you!)
- Forgetting to handle Option/Result
- Using clone() everywhere (understand borrowing first)
- Ignoring compiler warnings
- Not reading documentation
- Giving up too early (Rust has a learning curve)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- The Rust community for excellent documentation
- Cryptography researchers and educators
- Open source contributors
- Everyone learning Rust and crypto

## 📬 Contact

- GitHub Issues: [Report bugs or request features](https://github.com/yourusername/rust-crypto-learning/issues)
- Discussions: [Ask questions and share ideas](https://github.com/yourusername/rust-crypto-learning/discussions)

---

## 🎯 Quick Start Checklist

- [ ] Install Rust (`rustup`)
- [ ] Clone this repository
- [ ] Run `cargo check` to verify setup
- [ ] Complete variables-mutability example
- [ ] Read through ownership guide
- [ ] Try first cryptography example
- [ ] Complete one exercise
- [ ] Build your first mini-project
- [ ] Join Rust community
- [ ] Share your progress!

---

**Happy Learning! 🦀🔐**

Remember: The best way to learn is by doing. Start coding today, make mistakes, learn from them, and build amazing things!

*Last Updated: January 2026*
