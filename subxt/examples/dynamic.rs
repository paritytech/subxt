//! We can use dyanmic addresses/payloads to dynamically decode/encode things.
//! Prefer to avoid scale_value::Value and decode into well defined types where
//! possible.
use scale_decode::DecodeAsType;
use subxt::dynamic::{self, Value};
use subxt::{Error, OnlineClient, PolkadotConfig};

#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), Error> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<PolkadotConfig>::from_url("wss://rpc.polkadot.io").await?;
    let api_at_block = api.at_block(20_000_000u32).await?;

    //// Storage: TimeStamp.Now
    {
        // Here we create a storage address with no keys `()` and a `u64` value type.
        let timestamp_addr = dynamic::storage::<(), u64>("Timestamp", "Now");
        let timestamp_value = api_at_block.storage().fetch(timestamp_addr, ()).await?;

        // .decode() decodes as the default value type:
        let timestamp_u64 = timestamp_value.decode()?;
        println!("Block timestamp u64: {timestamp_u64}");

        // Despite the default u64 value type we can decode as something else if we prefer:
        let timestamp_val = timestamp_value.decode_as::<Value>()?;
        println!("Block timestamp Value: {timestamp_val}");
    }

    //// Storage: System.Account
    {
        // Let's define a type for our account details to decode into:
        #[derive(Debug, DecodeAsType)]
        struct AccountInfo {
            data: AccountInfoData,
        }
        #[derive(Debug, DecodeAsType)]
        struct AccountInfoData {
            // We only have to add the fields we're interested in:
            free: u128,
        }

        // Now, we set the expected value to the above type, and require one key
        // to be provided; a [u8;32] which corresponds to an AccountId32.
        let account_info_addr = dynamic::storage::<([u8; 32],), AccountInfo>("System", "Account");

        // Fetch this entry for some random AccountId:
        let account_id = demo_account_id();
        let account_value = api_at_block
            .storage()
            .fetch(account_info_addr, (account_id,))
            .await?;

        let account_details = account_value.decode()?;
        println!("Account details: {account_details:?}");
    }

    //// Constants
    {
        // Define the expected return type when creating the address:
        let deposit_addr = dynamic::constant::<u128>("Balances", "ExistentialDeposit");
        let deposit = api_at_block.constants().entry(deposit_addr)?;
        println!("Existential Deposit amount: {deposit}");

        // As with other dynamic address types, you can use a "catch-all" type
        // like scale_value::Value if you don't know what the type is, but this is
        // less performant so should be avoided if the flexibility isn't needed:
        let block_weights_addr = dynamic::constant::<Value>("System", "BlockWeights");
        let block_weights = api_at_block.constants().entry(block_weights_addr)?;

        // Using the Value type makes sense if you just want to print it out (and can be used
        // to then help you define a proper struct to decode a type into):
        println!("Block weights: {block_weights}");

        // Like this...
        #[derive(Debug, DecodeAsType)]
        struct BlockWeights {
            // Again; you can ignore all fields you don't care about;
            // DecodeAsType understands how to skip them for you.
            max_block: BlockWeightInfo,
        }
        #[derive(Debug, DecodeAsType)]
        struct BlockWeightInfo {
            ref_time: u128,
            proof_size: u128,
        }
        let block_weights_addr = dynamic::constant::<BlockWeights>("System", "BlockWeights");
        let block_weights = api_at_block.constants().entry(block_weights_addr)?;
        println!("Max total block weights: {:?}", block_weights.max_block);
    }

    //// Runtime APIs
    {
        // Define the arguments for this payload and the return type. Here we accept an
        // AccountId like type (32 bytes) and the return is a u32. Since we provide the
        // arguments in the payload, we can often omit the argument types.
        let runtime_api_payload = dynamic::runtime_api_call::<_, u32>(
            "AccountNonceApi",
            "account_nonce",
            (demo_account_id(),),
        );

        let account_nonce = api_at_block
            .runtime_apis()
            .call(runtime_api_payload)
            .await?;
        println!("Account nonce for demo account: {account_nonce}");
    }

    //// Transactions
    {
        use subxt_signer::sr25519::dev;
        let alice = dev::alice();
        let bob = dev::bob();

        // As with Runtime APIs, to submit a transaction you define the payload. There
        // is no response though; just the input values you wish to provide; here something
        // which looks like a `MultiAddress::Id([u8; 32])` and a value to transfer.
        let tx_payload = dynamic::transaction(
            "Balances",
            "transfer_keep_alive",
            (alice.public_key().to_address::<()>(), 12_000_000u128),
        );

        // This won't work when pointed at a real node since it uses a dev account:
        api_at_block
            .transactions()
            .sign_and_submit_then_watch_default(&tx_payload, &bob)
            .await?
            .wait_for_finalized_success()
            .await?;
    }

    Ok(())
}

fn demo_account_id() -> [u8; 32] {
    let account_id_hex = "9a4d0faa2ba8c3cc5711852960940793acf55bf195b6eecf88fa78e961d0ce4a";
    let account_id: [u8; 32] = hex::decode(account_id_hex).unwrap().try_into().unwrap();
    account_id
}
