# SOLyssey
**An on-chain educational treasure hunt on Solana that feels like a real Odyssey.**
Players interact with actual Solana programs, follow sequential paths filled with layered clues and riddles, and compete for real SOL prizes. The experience is designed to be fun and mysterious — the kind of challenge that appeals to people who love solving riddles, decoding clues, and uncovering secrets — while teaching practical skills that matter:
- Writing and calling Solana programs
- Debugging without public IDLs
- Interpreting custom errors
- Thinking about security
---
## Vision
Most learning resources for Solana are tutorials or documentation. SOLyssey turns the learning process into a game that feels like a genuine treasure hunt.
Each hunt has a theme inspired by real-world events and consists of six sequential paths. Progress is enforced on-chain through tickets and previous-path outputs, so players cannot skip ahead. The final path produces a verifiable on-chain claim. The first person to correctly submit it wins the prize.
---
## Current Status
Architecture is complete and the MVP has been clearly scoped for the MLH × Solana $1,000 Grant.  
Full implementation begins once the grant is secured.
---
## MVP Scope (funded by the $1,000 grant)
- Master program (`treasure_master`) with `start_here` instruction
- Path program (`treasure_path`) supporting 6 sequential paths with ticket-based progression
- One complete themed hunt
- Minimal discovery frontend + detailed tutorial and example scripts
- On-chain final claim proof mechanism
- Seed funding for the first prize pool
---
## High-Level Architecture
- **Master Program**: Entry point. The `start_here` instruction returns the hunt theme and first clue.
- **Path Program**: Handles all six paths. Each successful path creates a player-specific Ticket PDA and requires the previous path’s ticket as input.
- **Final Claim**: Path 6 creates a `FinalClaim` PDA that serves as on-chain proof for the prize.
More detailed architecture notes are available in the repository.
---
## Tech Stack
- Anchor (Solana programs)
- TypeScript / JavaScript (frontend and scripts)
- Minimal frontend focused on discovery and clear documentation
---
## About the Builder
Built by [jriff](https://jriffs.github.io/).
Previous Solana projects:
- **SolScope** – Solana wallet explorer
- **Drip Token** – Production-quality simple SPL token faucet on Devnet
Participant in the MLH 100 Days of Solana challenge (≈70% completion + competitions).
---
## License
MIT
