# Token Fundraiser

A modular smart contract for fundraising campaigns built with the Pinocchio.

## Overview

This project implements a decentralized fundraising system where:

- Users can create fundraising campaigns by defining a token to raise, a target amount, and duration.
- Contributors can participate by donating tokens to active campaigns.
- Campaign creators can claim the funds after completion.
- Contributors can request refunds.

## Architecture

The project uses the Pinocchio framework, a lightweight implementation for smart contracts. The project structure follows a modular pattern:

### Data Structure

**Fundraiser State** (`src/state/fundraiser.rs`):
- `current_amount`: Current amount raised
- `time_started`: Campaign start timestamp
- `maker`: Campaign creator (Pubkey)
- `mint_to_raise`: Token being raised
- `amount_to_raise`: Fundraising target
- `duration`: Campaign duration in days
- `bump`: PDA seed for signing

**Contributor State** (`src/state/contributor.rs`):
- `amount`: Total amount contributed by the participant

### Instructions

The contract supports four main operations:

1. **Initialize** (`src/instructions/initialize.rs`):
   - Creates a new fundraising campaign
   - Sets the creator, token, target, and duration
   - Initializes the start timestamp

2. **Contribute** (`src/instructions/contribute.rs`):
   - Allows users to contribute tokens to a campaign
   - Validates that the contribution is within limits (MIN_RAISE and MAX_RAISE)
   - Transfers tokens from the contributor's account to the campaign vault
   - Updates contribution records

3. **Check** (`src/instructions/check.rs`):
   - Allows the campaign creator to claim the raised funds
   - Verifies the validity of the fundraiser's PDA
   - Transfers all tokens from the vault to the creator's account

4. **Refund** (`src/instructions/refund.rs`):
   - Allows contributors to request a refund
   - Checks if there is an amount to be refunded
   - Transfers the tokens back to the contributor's account

## Design Decisions

1. **Security via PDAs**: 
   - Uses Program Derived Addresses (PDAs) for secure fund control
   - Implements signature verification for all token transfers

2. **Efficient Storage**:
   - Optimized memory layout with specific offsets for each field
   - Uses direct memory access for maximum efficiency

3. **Contribution Limits**:
   - Minimum: 10 units
   - Maximum: 1 quintillion (10^18) units

4. **Validations**:
   - Account owner verification
   - PDA validation before transfers
   - Balance checks before refunds
