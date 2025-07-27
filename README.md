# PulseBorrow

PulseBorrow is a Rust-based Solana program implementing the logic for a simple flash loan protocol using the [Anchor framework](https://book.anchor-lang.com/). The protocol allows borrowers to take out and repay flash loans within a single transaction, ensuring atomicity and safety using Solana's instruction sysvars.

---

## Features

- **Flash Loan Support:** Borrow funds with no collateral, provided they are repaid in the same transaction.
- **Borrow & Repay Instructions:** Two main instructions, `borrow` and `repay`, facilitate the loan lifecycle.
- **Secure Instruction Validation:** Uses Solana's instruction sysvar to validate the presence and correctness of borrow and repay instructions, preventing misuse.
- **Fee Mechanism:** Borrowed amount is charged with a hard-coded fee (currently 500 basis points, i.e., 5%).
- **Anchor & SPL Token Integration:** Uses Anchor macros for account validation and SPL Token for token transfers.

---

## Directory Structure

```
PulseBorrow/
├── programs/
│   └── blueshift_anchor_flash_loan/
│       ├── src/
│       │   ├── error.rs
│       │   ├── instructions/
│       │   │   ├── borrow.rs
│       │   │   ├── repay.rs
│       │   │   └── mod.rs
│       │   ├── states/
│       │   │   ├── loan.rs
│       │   │   └── mod.rs
│       │   └── lib.rs
├── tests/
│   └── blueshift_anchor_flash_loan.ts
├── migrations/
│   └── deploy.ts
```

---

## How It Works

### 1. Borrow Instruction

- The borrower calls the `borrow` instruction, specifying the amount to borrow.
- The protocol validates the request, ensures the amount is positive, and transfers tokens from the protocol account to the borrower’s associated token account.

### 2. Repay Instruction

- The borrower must call `repay` in the same transaction.
- The program checks the original borrow instruction and calculates the fee.
- The borrower repays the borrowed amount plus the fee, transferring tokens back to the protocol account.

### 3. Safety

- The program uses Solana's instruction sysvar to ensure the atomicity and order of borrow/repay operations.
- Custom error types ensure robust error handling for invalid instructions, missing accounts, and overflows.

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor](https://book.anchor-lang.com/getting-started/installation.html)
- Node.js & Yarn/NPM (for testing TypeScript scripts)

### Build & Deploy

1. **Clone the repository:**

   ```bash
   git clone https://github.com/raunit-dev/PulseBorrow.git
   cd PulseBorrow
   ```

2. **Build the Solana program:**

   ```bash
   anchor build
   ```

3. **Deploy to localnet:**

   ```bash
   anchor deploy
   ```

4. **Run tests:**

   ```bash
   anchor test
   ```

---

## Usage

- Use Anchor client to interact with the protocol via TypeScript or Rust.
- Example test interaction can be found in `tests/blueshift_anchor_flash_loan.ts`.

---

## Program Structure

- **Rust Program:** Located in `programs/blueshift_anchor_flash_loan/src/`
  - `instructions/borrow.rs` – Logic for borrowing.
  - `instructions/repay.rs` – Logic for repaying.
  - `states/loan.rs` – Account validation and context.
  - `error.rs` – Custom error codes for protocol safety.
- **Tests:** TypeScript-based tests using Anchor in `tests/`.
- **Migrations:** Deployment scripts in `migrations/`.

---

## Dependencies

- [Anchor framework](https://book.anchor-lang.com/)
- [Solana Program Library (SPL) Token](https://spl.solana.com/token)
- TypeScript (for client/testing)

---

## Notes

- The protocol is currently in development and may lack real-world safety guarantees.
- The fee is hard-coded to 5%; modify in code as needed for your use-case.
- No mainnet deployment configuration is provided yet.

---

## License

*No explicit license is declared. Please contact the repository owner for usage rights.*

---

## Author

- [raunit-dev](https://github.com/raunit-dev)

---

## Contributing

Pull requests and issues are welcome! Please open an issue to discuss your ideas or submit a PR.
