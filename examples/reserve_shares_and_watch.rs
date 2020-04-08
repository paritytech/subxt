use sp_keyring::AccountKeyring;
use substrate_subxt::{shares_atomic, system::System, ExtrinsicSuccess, SunTime as Runtime};

type AccountId = <Runtime as System>::AccountId;
type OrgId = <Runtime as shares_atomic::SharesAtomic>::OrgId;
type ShareId = <Runtime as shares_atomic::SharesAtomic>::ShareId;

fn main() {
    let result: Result<ExtrinsicSuccess<_>, Box<dyn std::error::Error + 'static>> =
        async_std::task::block_on(async move {
            env_logger::init();

            let alice_the_signer = AccountKeyring::Alice.pair();

            let reserves_alices_shares = AccountKeyring::Alice.to_account_id();

            let organization: OrgId = 1u64;
            let share_id: ShareId = 1u64;

            let cli = substrate_subxt::ClientBuilder::<Runtime>::new()
                .build()
                .await?;
            let xt = cli.xt(alice_the_signer, None).await?;
            // debugging
            println!("last message shown");
            let xt_result = xt
                .watch()
                .events_decoder(|decoder| {
                    // for any primitive event with no type size registered
                    decoder.register_type_size::<(u64, u64, u64)>("IdentificationTuple")
                })
                .submit(shares_atomic::reserve_shares::<Runtime>(
                    organization.into(),
                    share_id.into(),
                    reserves_alices_shares.clone().into(),
                ))
                .await?;
            // debugging
            println!("first message not shown");
            Ok(xt_result)
        });
    match result {
        Ok(extrinsic_success) => {
            match extrinsic_success
                .find_event::<(OrgId, ShareId, AccountId)>("SharesAtomic", "Reserve")
            {
                Some(Ok((org, share, account))) => println!(
                    "Account {:?} reserved id number {:?} shares for id number {:?} organization",
                    account, share, org
                ),
                Some(Err(err)) => println!("Failed to decode code hash: {}", err),
                None => println!("Failed to find SharesAtomic::Reserve Event"),
            }
        }
        Err(err) => println!("Error: {:?}", err),
    }
}
