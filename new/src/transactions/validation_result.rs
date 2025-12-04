use crate::error::ExtrinsicError;
use codec::Decode;

/// The result of performing [`SubmittableTransaction::validate()`].
#[derive(Clone, Debug, PartialEq)]
pub enum ValidationResult {
    /// The transaction is valid
    Valid(TransactionValid),
    /// The transaction is invalid
    Invalid(TransactionInvalid),
    /// Unable to validate the transaction
    Unknown(TransactionUnknown),
}

impl ValidationResult {
    /// Is the transaction valid.
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationResult::Valid(_))
    }

    #[allow(clippy::get_first)]
    pub(crate) fn try_from_bytes(bytes: Vec<u8>) -> Result<ValidationResult, ExtrinsicError> {
        // TaggedTransactionQueue_validate_transaction returns this:
        // https://github.com/paritytech/substrate/blob/0cdf7029017b70b7c83c21a4dc0aa1020e7914f6/primitives/runtime/src/transaction_validity.rs#L210
        // We copy some of the inner types and put the three states (valid, invalid, unknown) into one enum,
        // because from our perspective, the call was successful regardless.
        if bytes.get(0) == Some(&0) {
            // ok: valid. Decode but, for now we discard most of the information
            let res = TransactionValid::decode(&mut &bytes[1..])
                .map_err(ExtrinsicError::CannotDecodeValidationResult)?;
            Ok(ValidationResult::Valid(res))
        } else if bytes.get(0) == Some(&1) && bytes.get(1) == Some(&0) {
            // error: invalid
            let res = TransactionInvalid::decode(&mut &bytes[2..])
                .map_err(ExtrinsicError::CannotDecodeValidationResult)?;
            Ok(ValidationResult::Invalid(res))
        } else if bytes.get(0) == Some(&1) && bytes.get(1) == Some(&1) {
            // error: unknown
            let res = TransactionUnknown::decode(&mut &bytes[2..])
                .map_err(ExtrinsicError::CannotDecodeValidationResult)?;
            Ok(ValidationResult::Unknown(res))
        } else {
            // unable to decode the bytes; they aren't what we expect.
            Err(ExtrinsicError::UnexpectedValidationResultBytes(bytes))
        }
    }
}

/// Transaction is valid; here is some more information about it.
#[derive(Decode, Clone, Debug, PartialEq)]
pub struct TransactionValid {
    /// Priority of the transaction.
    ///
    /// Priority determines the ordering of two transactions that have all
    /// their dependencies (required tags) satisfied.
    pub priority: u64,
    /// Transaction dependencies
    ///
    /// A non-empty list signifies that some other transactions which provide
    /// given tags are required to be included before that one.
    pub requires: Vec<Vec<u8>>,
    /// Provided tags
    ///
    /// A list of tags this transaction provides. Successfully importing the transaction
    /// will enable other transactions that depend on (require) those tags to be included as well.
    /// Provided and required tags allow Substrate to build a dependency graph of transactions
    /// and import them in the right (linear) order.
    pub provides: Vec<Vec<u8>>,
    /// Transaction longevity
    ///
    /// Longevity describes minimum number of blocks the validity is correct.
    /// After this period transaction should be removed from the pool or revalidated.
    pub longevity: u64,
    /// A flag indicating if the transaction should be propagated to other peers.
    ///
    /// By setting `false` here the transaction will still be considered for
    /// including in blocks that are authored on the current node, but will
    /// never be sent to other peers.
    pub propagate: bool,
}

/// The runtime was unable to validate the transaction.
#[derive(Decode, Clone, Debug, PartialEq)]
pub enum TransactionUnknown {
    /// Could not lookup some information that is required to validate the transaction.
    CannotLookup,
    /// No validator found for the given unsigned transaction.
    NoUnsignedValidator,
    /// Any other custom unknown validity that is not covered by this enum.
    Custom(u8),
}

/// The transaction is invalid.
#[derive(Decode, Clone, Debug, PartialEq)]
pub enum TransactionInvalid {
    /// The call of the transaction is not expected.
    Call,
    /// General error to do with the inability to pay some fees (e.g. account balance too low).
    Payment,
    /// General error to do with the transaction not yet being valid (e.g. nonce too high).
    Future,
    /// General error to do with the transaction being outdated (e.g. nonce too low).
    Stale,
    /// General error to do with the transaction's proofs (e.g. signature).
    ///
    /// # Possible causes
    ///
    /// When using a signed extension that provides additional data for signing, it is required
    /// that the signing and the verifying side use the same additional data. Additional
    /// data will only be used to generate the signature, but will not be part of the transaction
    /// itself. As the verifying side does not know which additional data was used while signing
    /// it will only be able to assume a bad signature and cannot express a more meaningful error.
    BadProof,
    /// The transaction birth block is ancient.
    ///
    /// # Possible causes
    ///
    /// For `FRAME`-based runtimes this would be caused by `current block number`
    /// - Era::birth block number > BlockHashCount`. (e.g. in Polkadot `BlockHashCount` = 2400, so
    ///   a transaction with birth block number 1337 would be valid up until block number 1337 + 2400,
    ///   after which point the transaction would be considered to have an ancient birth block.)
    AncientBirthBlock,
    /// The transaction would exhaust the resources of current block.
    ///
    /// The transaction might be valid, but there are not enough resources
    /// left in the current block.
    ExhaustsResources,
    /// Any other custom invalid validity that is not covered by this enum.
    Custom(u8),
    /// An transaction with a Mandatory dispatch resulted in Error. This is indicative of either a
    /// malicious validator or a buggy `provide_inherent`. In any case, it can result in
    /// dangerously overweight blocks and therefore if found, invalidates the block.
    BadMandatory,
    /// An transaction with a mandatory dispatch tried to be validated.
    /// This is invalid; only inherent transactions are allowed to have mandatory dispatches.
    MandatoryValidation,
    /// The sending address is disabled or known to be invalid.
    BadSigner,
}
