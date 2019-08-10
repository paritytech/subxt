use crate::{
    codec::compact,
    error::Error,
    srml::system::System,
    Client,
    XtBuilder,
};
use futures::future::Future;
use parity_scale_codec::Codec;
use runtime_primitives::traits::{
    MaybeSerializeDebug,
    Member,
    SimpleArithmetic,
    StaticLookup,
};
use runtime_support::Parameter;
use substrate_primitives::Pair;

pub trait Balances: System {
    type Balance: Parameter
        + Member
        + SimpleArithmetic
        + Codec
        + Default
        + Copy
        + MaybeSerializeDebug
        + From<<Self as System>::BlockNumber>;
}

pub trait BalancesStore {
    type Balances: Balances;

    fn free_balance(
        &self,
        account_id: <Self::Balances as System>::AccountId,
    ) -> Box<dyn Future<Item = <Self::Balances as Balances>::Balance, Error = Error> + Send>;
}

pub trait BalancesCalls {
    type Balances: Balances;

    fn transfer(
        &mut self,
        to: <<Self::Balances as System>::Lookup as StaticLookup>::Source,
        amount: <Self::Balances as Balances>::Balance,
    ) -> Box<dyn Future<Item = <Self::Balances as System>::Hash, Error = Error> + Send>;
}

impl<T: Balances + 'static> BalancesStore for Client<T> {
    type Balances = T;

    fn free_balance(
        &self,
        account_id: <Self::Balances as System>::AccountId,
    ) -> Box<dyn Future<Item = <Self::Balances as Balances>::Balance, Error = Error> + Send>
    {
        let free_balance_map = || {
            Ok(self
                .metadata()
                .module("Balances")?
                .storage("FreeBalance")?
                .get_map::<
                <Self::Balances as System>::AccountId,
                <Self::Balances as Balances>::Balance>()?)
        };
        let map = match free_balance_map() {
            Ok(map) => map,
            Err(err) => return Box::new(futures::future::err(err)),
        };
        Box::new(self.fetch_or(map.key(account_id), map.default()))
    }
}

impl<T: Balances + 'static, P> BalancesCalls for XtBuilder<T, P>
where
    P: Pair,
    P::Public: Into<<<T as System>::Lookup as StaticLookup>::Source>,
    P::Signature: Codec,
{
    type Balances = T;

    fn transfer(
        &mut self,
        to: <<Self::Balances as System>::Lookup as StaticLookup>::Source,
        amount: <Self::Balances as Balances>::Balance,
    ) -> Box<dyn Future<Item = <Self::Balances as System>::Hash, Error = Error> + Send>
    {
        let transfer_call = || {
            Ok(self
                .metadata()
                .module("Balances")?
                .call("transfer", (to, compact(amount)))?)
        };
        let call = match transfer_call() {
            Ok(call) => call,
            Err(err) => return Box::new(futures::future::err(err)),
        };
        Box::new(self.submit(call))
    }
}
