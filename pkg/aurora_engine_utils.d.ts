/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} bytes
* @param {bigint | undefined} [block_height]
* @returns {JsEthTransactionKind}
*/
export function parseTransaction(bytes: Uint8Array, block_height?: bigint): JsEthTransactionKind;
export interface JsTransaction1559 {
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

export interface JsSignedTransaction1559 {
    transaction: JsTransaction1559;
    parity: number;
    r: string;
    s: string;
}

export interface JsAccessTuple {
    address: string;
    storageKeys: string[];
}

export interface JsTransaction2930 {
    chainId: string;
    nonce: string;
    gasPrice: string;
    gasLimit: string;
    to: string;
    value: string;
    data: number[];
    accessList: JsAccessTuple[];
}

export interface JsSignedTransaction2930 {
    transaction: JsTransaction2930;
    parity: number;
    r: string;
    s: string;
}

export interface JsTransactionLegacy {
    nonce: string;
    gasPrice: string;
    gasLimit: string;
    to: string;
    value: string;
    data: number[];
}

export interface JsLegacyEthSignedTransaction {
    transaction: JsTransactionLegacy;
    v: string;
    r: string;
    s: string;
}

export type JsEthTransactionKind = { Legacy: JsLegacyEthSignedTransaction } | { Eip2930: JsSignedTransaction2930 } | { Eip1559: JsSignedTransaction1559 };

