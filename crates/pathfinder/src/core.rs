//! Contains core functions and types that are widely used but have no real
//! home of their own.
//!
//! This includes many trivial wrappers around [StarkHash] which help by providing additional type safety.
use pedersen::StarkHash;
use serde::{Deserialize, Serialize};
use web3::types::{H160, H256};

/// The address of a StarkNet contract.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ContractAddress(pub StarkHash);

/// The salt of a StarkNet contract address.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct ContractAddressSalt(pub StarkHash);

/// A StarkNet contract's hash. This is a hash over a contract's
/// deployment properties e.g. code and ABI.
///
/// Not to be confused with [ContractStateHash].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContractHash(pub StarkHash);

/// A StarkNet contract's state hash. This is the value stored
/// in the global state tree.
///
/// Not to be confused with [ContractHash].
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractStateHash(pub StarkHash);

/// A commitment root of a StarkNet contract. This is the entry-point
/// for a contract's state at a specific point in time via the contract
/// state tree.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractRoot(pub StarkHash);

/// A Starknet contract's bytecode and ABI.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractCode {
    pub bytecode: Vec<ByteCodeWord>,
    pub abi: String,
}

/// Entry point of a StarkNet `call`.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct EntryPoint(pub StarkHash);

impl EntryPoint {
    /// Returns a new EntryPoint which has been truncated to fit from Keccak256 digest of input.
    ///
    /// See: <https://starknet.io/documentation/contracts/#function_selector>
    pub fn hashed(input: &[u8]) -> Self {
        use sha3::Digest;
        EntryPoint(crate::state::contract_hash::truncated_keccak(
            <[u8; 32]>::from(sha3::Keccak256::digest(input)),
        ))
    }
}

/// A single parameter passed to a StarkNet `call`.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct CallParam(pub StarkHash);

/// A single parameter passed to a StarkNet contract constructor.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct ConstructorParam(pub StarkHash);

/// A single result value of a StarkNet `call`.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct CallResultValue(pub StarkHash);

/// A single element of a signature used to secure a StarkNet `call`.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct CallSignatureElem(pub StarkHash);

/// A word from a StarkNet contract bytecode.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct ByteCodeWord(pub StarkHash);

/// The address of a storage element for a StarkNet contract.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct StorageAddress(pub StarkHash);

/// The value of a storage element for a StarkNet contract.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct StorageValue(pub StarkHash);

/// A commitment root of the global StarkNet state. This is the entry-point
/// for the global state at a specific point in time via the global state tree.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct GlobalRoot(pub StarkHash);

/// A StarkNet block hash.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct StarknetBlockHash(pub StarkHash);

/// A StarkNet block number.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct StarknetBlockNumber(pub u64);

/// The timestamp of a Starknet block.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StarknetBlockTimestamp(pub u64);

/// A StarkNet transaction hash.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct StarknetTransactionHash(pub StarkHash);

/// A StarkNet transaction hash.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct StarknetTransactionIndex(pub u64);

/// A single element of a signature used to secure a StarkNet transaction.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct TransactionSignatureElem(pub StarkHash);

/// A nonce that is added to an L1 to L2 message in a StarkNet transaction.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct L1ToL2MessageNonce(pub StarkHash);

/// A single element of the payload of an L1 to L2 message in a StarkNet transaction.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct L1ToL2MessagePayloadElem(pub StarkHash);

/// A single element of the payload of an L2 to L1 message in a StarkNet transaction.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct L2ToL1MessagePayloadElem(pub StarkHash);

/// StarkNet transaction event data.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct EventData(pub StarkHash);

/// StarkNet transaction event key.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct EventKey(pub StarkHash);

/// StarkNet protocol version.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct StarknetProtocolVersion(pub H256);

/// An Ethereum address.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct EthereumAddress(pub H160);

/// An Ethereum block hash.
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct EthereumBlockHash(pub H256);

/// An Ethereum block number.
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct EthereumBlockNumber(pub u64);

/// An Ethereum transaction hash.
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct EthereumTransactionHash(pub H256);

/// An Ethereum transaction's index within a block.
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct EthereumTransactionIndex(pub u64);

/// An Ethereum log's index within a block.
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct EthereumLogIndex(pub u64);

impl StarknetBlockNumber {
    pub const GENESIS: StarknetBlockNumber = StarknetBlockNumber(0);
}

impl std::cmp::PartialOrd for StarknetBlockNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl std::ops::Add<u64> for StarknetBlockNumber {
    type Output = StarknetBlockNumber;

    fn add(self, rhs: u64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl std::ops::AddAssign<u64> for StarknetBlockNumber {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}

impl std::ops::Sub<u64> for StarknetBlockNumber {
    type Output = StarknetBlockNumber;

    fn sub(self, rhs: u64) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl std::ops::SubAssign<u64> for StarknetBlockNumber {
    fn sub_assign(&mut self, rhs: u64) {
        self.0 -= rhs;
    }
}

impl From<EthereumBlockNumber> for web3::types::BlockId {
    fn from(number: EthereumBlockNumber) -> Self {
        web3::types::BlockId::Number(web3::types::BlockNumber::Number(number.0.into()))
    }
}

impl From<StarknetBlockNumber> for crate::rpc::types::BlockNumberOrTag {
    fn from(number: StarknetBlockNumber) -> Self {
        crate::rpc::types::BlockNumberOrTag::Number(number)
    }
}

impl From<StarknetBlockHash> for crate::rpc::types::BlockHashOrTag {
    fn from(hash: StarknetBlockHash) -> Self {
        crate::rpc::types::BlockHashOrTag::Hash(hash)
    }
}
