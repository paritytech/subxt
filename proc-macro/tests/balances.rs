use codec::{
    Decode,
    Encode,
};
use frame_support::Parameter;
use sp_keyring::AccountKeyring;
use sp_runtime::traits::{
    AtLeast32Bit,
    MaybeSerialize,
    Member,
};
use std::fmt::Debug;
use substrate_subxt::{
    system::System,
    ClientBuilder,
    KusamaRuntime,
};
use substrate_subxt_proc_macro::{
    module,
    storage,
    Call,
    Event,
};

#[module]
pub trait Balances: System {
    type Balance: Parameter
        + Member
        + AtLeast32Bit
        + codec::Codec
        + Default
        + Copy
        + MaybeSerialize
        + Debug
        + From<<Self as System>::BlockNumber>;
}

// generates:
//
// const MODULE: &str = "Balances";
//
// put trait BalancesEventsDecoder {
//     fn with_balances(self) -> Self;
// }
//
// impl<T: Balances, P, S, E> BalancesEventsDecoder for EventsSubscriber<T, P, S, E> {
//     fn with_balances(self) -> Self {
//         self.events_decoder(|decoder| {
//             decoder.register_type_size::<T::Balance>("Balance")
//         }
//     }
// }

#[derive(Call, Encode)]
pub struct TransferCall<T: Balances> {
    to: <T as System>::Address,
    #[codec(compact)]
    amount: T::Balance,
}

// generates:
//
// pub fn transfer<T: Balances>(
//     to: <T as System>::Address,
//     amount: T::Balance,
// ) -> Call<Transfer<T>> {
//     Call::new(MODULE, "transfer", Transfer { to, amount })
// }

#[derive(Decode, Event)]
pub struct TransferEvent<T: Balances> {
    pub from: <T as System>::AccountId,
    pub to: <T as System>::AccountId,
    pub amount: T::Balance,
}

// generates:
//
// pub trait TransferEventExt<T: Balances> {
//     fn transfer(&self) -> Option<Result<TransferEvent<T>, codec::Error>>;
// }
//
// impl<T: Balances> TransferEventExt<T: Balances> for ExtrisicSuccess<T> {
//     fn transfer(&self) -> Option<Result<TransferEvent<T>, codec::Error>> {
//         self.find_event(MODULE, "transfer")
//     }
// }

#[derive(Clone, Decode, Default)]
pub struct AccountData<Balance> {
    pub free: Balance,
    pub reserved: Balance,
    pub misc_frozen: Balance,
    pub fee_frozen: Balance,
}

#[storage]
pub trait BalancesStore<T: Balances> {
    fn account(account_id: &<T as System>::AccountId) -> AccountData<T::Balance>;
}

// generates:
//
// pub trait BalancesStore<T: Balances> {
//     fn account<'a>(
//         &'a self,
//         account_id: &<T as System>::AccountId,
//     ) -> Pin<Box<dyn Future<Output = Result<AccountData<T::Balance>, Error>> + Send + 'a>>;
// }
//
// impl<T, S, E> BalancesStore<T> for Client<T, S, E>
// where
//     T: Balances + Send + Sync,
//     S: 'static,
//     E: Send + Sync + 'static,
// {
//     fn account<'a>(
//         &'a self,
//         account_id: &<T as System>::AccountId,
//     ) -> Pin<Box<dyn Future<Output = Result<AccountData<T::Balance>, Error>> + Send + 'a>> {
//         let store_fn = || {
//             Ok(self.metadata().module(MODULE)?.storage("Account")?.map()?)
//         };
//         let store = match store_fn() {
//             Ok(v) => v,
//             Err(e) => return Box::pin(future::err(e)),
//         };
//         let future = self.fetch(store.key(account_id), None);
//         Box::pin(async move {
//             let v = if let Some(v) = future.await? {
//                 Some(v)
//             } else {
//                 store.default().cloned()
//             };
//             Ok(v.unwrap_or_default())
//         })
//     }

impl Balances for KusamaRuntime {
    type Balance = u128;
}

#[async_std::test]
#[ignore]
async fn test_balances() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::<KusamaRuntime>::new().build().await?;

    let alice_balance = client
        .account(&AccountKeyring::Alice.to_account_id())
        .await?
        .free;
    let bob_balance = client
        .account(&AccountKeyring::Bob.to_account_id())
        .await?
        .free;

    let transfer_event = client
        .xt(AccountKeyring::Alice.pair(), None)
        .await?
        .watch()
        .with_balances()
        .transfer(AccountKeyring::Bob.to_account_id(), 10_000)
        .await?
        .transfer()
        .unwrap()?;

    assert_eq!(transfer_event.from, AccountKeyring::Alice.to_account_id());
    assert_eq!(transfer_event.to, AccountKeyring::Bob.to_account_id());
    assert_eq!(transfer_event.amount, 10_000);

    let new_alice_balance = client
        .account(&AccountKeyring::Alice.to_account_id())
        .await?
        .free;
    let new_bob_balance = client
        .account(&AccountKeyring::Bob.to_account_id())
        .await?
        .free;

    assert_eq!(new_alice_balance, alice_balance - 10_000);
    assert_eq!(new_bob_balance, bob_balance + 10_000);

    Ok(())
}
