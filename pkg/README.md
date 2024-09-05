# Aurora Engine Utilities for JavaScript

This package provides utilities for parsing Aurora transactions in JavaScript applications. It supports various Ethereum
transaction types and simplifies the process of working with Aurora transactions within the NEAR ecosystem.

## Table of Contents

- [Installation](#installation)
- [Features](#features)
- [Usage](#usage)
    - [Parsing Transactions](#parsing-transactions)
- [API Reference](#api-reference)
    - [Functions](#functions)
    - [Types](#types)

## Installation

Install the package using npm:

```bash
npm install @cawabunga/aurora-engine-utils-js
```

## Features

- Parse Aurora transactions from raw bytes
- Support for Legacy, EIP-2930, and EIP-1559 transaction types
- Optional block height parameter for context-aware parsing

## Usage

### Parsing Transactions

```javascript
import {parseTransaction} from '@cawabunga/aurora-engine-utils-js';
import {connect} from 'near-api-js';

// Connect to NEAR network
const near = await connect(config);
const account = await near.account("example.testnet");

// Get a transaction outcome
const outcome = await account.connection.provider.txStatus(
    'HWS862NvR9MX1Rx6ht1vE9EKTbrmSwtQf4dVYf3oSsuo',
    'aurora'
);

// Extract the raw transaction bytes
const rawBytes = Buffer.from(
    outcome.transaction.actions[0].FunctionCall.args,
    'base64'
);

// Optional: Get the block height
const blockHeight = BigInt(outcome.transaction_outcome.block_height);

// Parse the transaction
const parsedTransaction = parseTransaction(rawBytes, blockHeight);

console.log(parsedTransaction);
```

The `parsedTransaction` object will be of type `JsEthTransactionKind`, which can be one of three transaction types:
Legacy, EIP-2930, or EIP-1559.

## API Reference

### Functions

#### `parseTransaction(bytes: Uint8Array, block_height?: bigint): JsEthTransactionKind`

Parses an encoded Aurora transaction and returns a `JsEthTransactionKind` object.

- `bytes`: A Uint8Array containing the raw transaction data.
- `block_height` (optional): A bigint representing the block height for context-aware parsing.
- Returns: A `JsEthTransactionKind` object representing the decoded transaction.

### Types

#### `JsEthTransactionKind`

A union type representing different Ethereum transaction kinds:

```typescript
type JsEthTransactionKind =
    { Legacy: JsLegacyEthSignedTransaction } |
    { Eip2930: JsSignedTransaction2930 } |
    { Eip1559: JsSignedTransaction1559 };
```

#### `JsLegacyEthSignedTransaction`

Represents a signed legacy Ethereum transaction:

```typescript
interface JsLegacyEthSignedTransaction {
    transaction: JsTransactionLegacy;
    v: string;
    r: string;
    s: string;
}

interface JsTransactionLegacy {
    nonce: string;
    gasPrice: string;
    gasLimit: string;
    to: string;
    value: string;
    data: number[];
}
```

#### `JsSignedTransaction2930`

Represents a signed EIP-2930 transaction:

```typescript
interface JsSignedTransaction2930 {
    transaction: JsTransaction2930;
    parity: number;
    r: string;
    s: string;
}

interface JsTransaction2930 {
    chainId: string;
    nonce: string;
    gasPrice: string;
    gasLimit: string;
    to: string;
    value: string;
    data: number[];
    accessList: JsAccessTuple[];
}
```

#### `JsSignedTransaction1559`

Represents a signed EIP-1559 transaction:

```typescript
interface JsSignedTransaction1559 {
    transaction: JsTransaction1559;
    parity: number;
    r: string;
    s: string;
}

interface JsTransaction1559 {
    chainId: string;
    nonce: string;
    maxPriorityFeePerGas: string;
    maxFeePerGas: string;
    gasLimit: string;
    to: string;
    value: string;
    data: number[];
    accessList: JsAccessTuple[];
}
```

#### `JsAccessTuple`

Represents an access list item:

```typescript
interface JsAccessTuple {
    address: string;
    storageKeys: string[];
}
```
