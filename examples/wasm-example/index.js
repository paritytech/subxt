/// The `@polkadot/extension-dapp` package can be dynamically imported.
/// Usually it is wise to use a package manager like npm or yarn to install it as a dependency.
/// The `getDAppMod`

/**
 * The `@polkadot/extension-dapp` package can be dynamically imported.
 * Usually it is wise to use a package manager like npm or yarn to install it as a dependency.
 *
 * The `getDAppMod` closure returns the `@polkadot/extension-dapp` module on demand. 
 */
let getDAppMod = (() => {
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
 * Queries wallets from browser extensions like Talisman and the Polkadot.js extension for user accounts.
 *
 *  @returns a json string that contains all the accounts that were found.
 */
async function getAccounts() {
    const dApp = await getDAppMod();
    await dApp.web3Enable("Subxt Example App");
    const allAccounts = await dApp.web3Accounts();
    const accountObjects = allAccounts.map((account) => ({
        name: account.meta.name,
        source: account.meta.source,
        ty: account.type,
        address: account.address
    }));
    return JSON.stringify(accountObjects);
}

/**
 *
 * @param {string} hex_message hex encoded message that should be signed
 * @param {string} account json string that contains an account object with this structure:
 *
 * {
    name: String 
    source: String,
    ty: String,
    address: String
    }

 The source is something like talisman or polkadotjs and can be used to get the signer for this extension.
 */
async function signHexMessage(hex_message, account) {
    const account_obj = JSON.parse(account);
    const dApp = await getDAppMod();
    const injector = await dApp.web3FromSource(account_obj.source);
    const signRaw = injector?.signer?.signRaw;
    if (!!signRaw) {
        const {signature} = await signRaw({
            address: account_obj.address,
            data: hex_message,
            type: "bytes",
        });
        return signature;
    } else {
        throw "The extension's injector does not have a `signRaw` function on its `signer`";
    }
}
