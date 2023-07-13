/// The `@polkadot/extension-dapp` package can be dynamically imported.
/// Usually it is wise to use a package manager like npm or yarn to install it as a dependency.
/// The `getPolkadotJsExtensionMod`

/**
 * The `@polkadot/extension-dapp` package can be dynamically imported.
 * Usually it is wise to use a package manager like npm or yarn to install it as a dependency.
 *
 * The `getPolkadotJsExtensionMod` closure returns the `@polkadot/extension-dapp` module on demand.
 */
let getPolkadotJsExtensionMod = (() => {
    let mod = null;

    // initialize `@polkadot/extension-dapp` module on page load
    let initPromise = (async () => {
        mod = await import(
            "https://cdn.jsdelivr.net/npm/@polkadot/extension-dapp@0.46.3/+esm"
            );
    })();

    // return a function that waits for initialization to be finished, in case mod is not initialized yet.
    return async () => {
        if (mod == null) {
            await initPromise;
        }
        return mod;
    };
})();

/**
 *  Queries wallets from browser extensions like Talisman and the Polkadot.js extension for user accounts.
 *
 *  @returns a json string that contains all the accounts that were found.
 */
async function getAccounts() {
    const extensionMod = await getPolkadotJsExtensionMod();
    await extensionMod.web3Enable("Subxt Example App");
    const allAccounts = await extensionMod.web3Accounts();
    const accountObjects = allAccounts.map((account) => ({
        name: account.meta.name, // e.g. "Alice"
        source: account.meta.source, // e.g. "talisman", "polkadot-js"
        ty: account.type, // e.g. "sr25519"
        address: account.address // e.g. "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
    }));
    return JSON.stringify(accountObjects);
}

/**
 *
 * @param {string} hex_message hex encoded message that should be signed: for example for Hello => "0x48656c6c6f"
 * @param {string} source browser extension used for signing that contains account with `address`
 * @param {string} address SS58 formatted address
 */
async function signHexMessage(hex_message, source, address) {
    const extensionMod = await getPolkadotJsExtensionMod();
    const injector = await extensionMod.web3FromSource(source);
    const signRaw = injector?.signer?.signRaw;
    if (!!signRaw) {
        const {signature} = await signRaw({
            address: address,
            data: hex_message,
            type: "bytes",
        });
        return signature;
    } else {
        throw "The extension's injector does not have a `signRaw` function on its `signer`";
    }
}

//
// let payload = {
//     "specVersion": "0x000024d6",
//     "transactionVersion": "0x00000018",
//     "address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
//     "blockHash": "0xd7aad6185db012b7ffbce710b55234d6c9589170566b925ee50cfa3d7f1e6f8f",
//     "blockNumber": "0x00000000",
//     "era": "0x0000",
//     "genesisHash": "0xd7aad6185db012b7ffbce710b55234d6c9589170566b925ee50cfa3d7f1e6f8f",
//     "method": "0x0503001cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c0b00c465f14670",
//     "nonce": "0x00000000",
//     "signedExtensions": [
//         "CheckNonZeroSender",
//         "CheckSpecVersion",
//         "CheckTxVersion",
//         "CheckGenesis",
//         "CheckMortality",
//         "CheckNonce",
//         "CheckWeight",
//         "ChargeTransactionPayment",
//         "PrevalidateAttests"
//     ],
//     "tip": "0x00000000000000000000000000000000",
//     "version": 4
// };
async function signPayload(payloadAsStr, source, address) {
    let payload = JSON.parse(payloadAsStr);
    console.log("js: payload:", payload);

    //
    // payload = {
    //     specVersion: "0x000024d6",
    //     transactionVersion: "0x00000018",
    //     address: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    //     blockHash:
    //         "0x8622e16cc8a143898e8ac6acc3036cd9d8a0c54640fde142354a422c025bc1ce",
    //     blockNumber: "0x00000000",
    //     era: "0x0000",
    //     genesisHash:
    //         "0x8622e16cc8a143898e8ac6acc3036cd9d8a0c54640fde142354a422c025bc1ce",
    //     method:
    //         "0x00001448656c6c6f",
    //     //    "0x0503001cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c0b00c465f14670",
    //     nonce: "0x00000000",
    //     signedExtensions: [
    //         "CheckNonZeroSender",
    //         "CheckSpecVersion",
    //         "CheckTxVersion",
    //         "CheckGenesis",
    //         "CheckMortality",
    //         "CheckNonce",
    //         "CheckWeight",
    //         "ChargeTransactionPayment",
    //         "PrevalidateAttests",
    //     ],
    //     tip: "0x00000000000000000000000000000000",
    //     version: 4,
    // };

    const extensionMod = await getPolkadotJsExtensionMod();
    const injector = await extensionMod.web3FromSource(source);
    const signPayload = injector?.signer?.signPayload;
    if (!!signPayload) {
        const {signature} = await signPayload(payload);
        console.log("signature js:", signature)
        return signature;
    } else {
        throw "The extension's injector does not have a `signPayload` function on its `signer`";
    }
    return "hello";
}
