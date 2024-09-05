# Aurora Engine Utilities for JavaScript

This package contains a collection of utilities for working with the Aurora Engine in JavaScript.

## Installation

```bash
npm install @cawabunga/aurora-engine-utils-js
```

## Usage

```javascript
import {parseTransaction} from '@cawabunga/aurora-engine-utils-js';

// Parse a transaction from an execution outcome
const outcome = await nearRPC.txStatus('HWS862NvR9MX1Rx6ht1vE9EKTbrmSwtQf4dVYf3oSsuo', 'aurora')
const encodedTransaction = Buffer.from(
    outcome.transaction.actions[0].FunctionCall.args,
    'base64',
)
const parsedTransaction = parseTransaction(encodedTransaction)
// And then do whatever you want with the parsed transaction

// Also possible to parse transactions from a chunk
const chunk = await nearRpc.chunk(someChunkHash)
const parsedTransactions = chunk.transactions
    .filter((tx) => tx.receiver_id === 'aurora')
    .flatMap((tx) => tx.actions)
    .filter((action) => 'FunctionCall' in action)
    .map((action) => {
        const encodedTransaction = Buffer.from(action.FunctionCall.args, 'base64')
        return parseTransaction(encodedTransaction)
    })
```