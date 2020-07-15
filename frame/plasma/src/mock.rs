//! Test utilities

#![cfg(test)]

use super::*;
pub use frame_support::{
    impl_outer_dispatch, impl_outer_event, impl_outer_origin, parameter_types,
    traits::OnFinalize,
    weights::{WeightToFeeCoefficients, WeightToFeePolynomial},
};
pub use hex_literal::hex;
pub use pallet_balances as balances;
pub use pallet_contracts::{self as contracts, ContractAddressFor, TrieId, TrieIdGenerator};
pub use pallet_ovm::{self as ovm, AtomicPredicateIdConfig};
use sp_core::crypto::AccountId32;
pub use sp_core::{crypto::key_types, H256};
pub use sp_runtime::testing::{Header, UintAuthorityId};
pub use sp_runtime::traits::{BlakeTwo256, ConvertInto, IdentifyAccount, IdentityLookup};
pub use sp_runtime::{KeyTypeId, Perbill};

pub type BlockNumber = u64;
pub type AccountId = AccountId32;
pub type Balance = u64;

lazy_static::lazy_static! {
    pub static ref ALICE_STASH: AccountId = to_account_from_seed(&hex![
        "0000000000000000000000000000000000000000000000000000000000005553"
    ]);
        pub static ref BOB_STASH: AccountId = to_account_from_seed(&hex![
        "0000000000000000000000000000000000000000000000000000000000008553"
    ]);
        pub static ref CHARLIE_STASH: AccountId = to_account_from_seed(&hex![
        "0000000000000000000000000000000000000000000000000000000000009553"
    ]);
}

impl_outer_origin! {
    pub enum Origin for Test  where system = frame_system {}
}

impl_outer_dispatch! {
    pub enum Call for Test where origin: Origin {
        pallet_balances::Balances,
        pallet_contracts::Contracts,
    }
}

mod plasma {
    // Re-export contents of the root. This basically
    // needs to give a name for the current crate.
    // This hack is required for `impl_outer_event!`.
    pub use super::super::*;
    use frame_support::impl_outer_event;
}

impl_outer_event! {
    pub enum MetaEvent for Test {
        system<T>,
        balances<T>,
        contracts<T>,
        ovm<T>,
        plasma<T>,
    }
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut storage = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    let _ = pallet_balances::GenesisConfig::<Test> {
        balances: vec![(ALICE_STASH, 1_000_000_000_000_000_000)],
    }
    .assimilate_storage(&mut storage);

    let _ = ovm::GenesisConfig {
        current_schedule: Default::default(),
    }
    .assimilate_storage(&mut storage);

    storage.into()
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Test;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: u32 = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::one();
}

impl frame_system::Trait for Test {
    type Origin = Origin;
    type BaseCallFilter = ();
    type Index = u64;
    type BlockNumber = BlockNumber;
    type Call = Call;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = MetaEvent;
    type DbWeight = ();
    type BlockHashCount = BlockHashCount;
    type BlockExecutionWeight = ();
    type ExtrinsicBaseWeight = ();
    type MaximumExtrinsicWeight = ();
    type MaximumBlockWeight = MaximumBlockWeight;
    type MaximumBlockLength = MaximumBlockLength;
    type AvailableBlockRatio = AvailableBlockRatio;
    type Version = ();
    type ModuleToIndex = ();
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
}

parameter_types! {
    pub const ExistentialDeposit: Balance = 10;
}

impl pallet_balances::Trait for Test {
    type Balance = Balance;
    type Event = MetaEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = frame_system::Module<Test>;
}

parameter_types! {
    pub const MinimumPeriod: u64 = 1;
}

impl pallet_timestamp::Trait for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
}

pub struct WeightToFee;
impl WeightToFeePolynomial for WeightToFee {
    type Balance = u64;
    fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
        Default::default()
    }
}

impl pallet_transaction_payment::Trait for Test {
    type Currency = Balances;
    type OnTransactionPayment = ();
    type TransactionByteFee = TransactionByteFee;
    type WeightToFee = WeightToFee;
    type FeeMultiplierUpdate = ();
}

parameter_types! {
    pub const SignedClaimHandicap: u64 = 2;
    pub const TombstoneDeposit: u64 = 16;
    pub const StorageSizeOffset: u32 = 8;
    pub const RentByteFee: u64 = 4;
    pub const RentDepositOffset: u64 = 10_000;
    pub const SurchargeReward: u64 = 150;
    pub const TransactionBaseFee: u64 = 2;
    pub const TransactionByteFee: u64 = 6;
    pub const ContractFee: u64 = 21;
    pub const CallBaseFee: u64 = 135;
    pub const InstantiateBaseFee: u64 = 175;
    pub const MaxDepth: u32 = 100;
    pub const MaxValueSize: u32 = 16_384;
}

impl pallet_contracts::Trait for Test {
    type Time = Timestamp;
    type Randomness = Randomness;
    type Currency = Balances;
    type Event = MetaEvent;
    type DetermineContractAddress = pallet_contracts::SimpleAddressDeterminer<Test>;
    type TrieIdGenerator = pallet_contracts::TrieIdFromParentCounter<Test>;
    type RentPayment = ();
    type SignedClaimHandicap = SignedClaimHandicap;
    type TombstoneDeposit = TombstoneDeposit;
    type StorageSizeOffset = StorageSizeOffset;
    type RentByteFee = RentByteFee;
    type RentDepositOffset = RentDepositOffset;
    type SurchargeReward = SurchargeReward;
    type MaxDepth = MaxDepth;
    type MaxValueSize = MaxValueSize;
    type WeightPrice = ();
}

parameter_types! {
    pub const DisputePeriod: BlockNumber = 7;
}

lazy_static::lazy_static! {
    pub static ref NOT_ADDRESS: AccountId = to_account_from_seed(&hex![
        "0000000000000000000000000000000000000000000000000000000000000003"
    ]);
    pub static ref AND_ADDRESS: AccountId = to_account_from_seed(&hex![
        "0000000000000000000000000000000000000000000000000000000000000004"
    ]);
    pub static ref OR_ADDRESS: AccountId = to_account_from_seed(&hex![
        "0000000000000000000000000000000000000000000000000000000000000005"
    ]);
    pub static ref FOR_ALL_ADDRESS: AccountId = to_account_from_seed(&hex![
        "0000000000000000000000000000000000000000000000000000000000000006"
    ]);
    pub static ref THERE_EXISTS_ADDRESS: AccountId = to_account_from_seed(&hex![
        "0000000000000000000000000000000000000000000000000000000000000007"
    ]);
    pub static ref EQUAL_ADDRESS: AccountId = to_account_from_seed(&hex![
        "0000000000000000000000000000000000000000000000000000000000000008"
    ]);
    pub static ref IS_CONTAINED_ADDRESS: AccountId = to_account_from_seed(&hex![
        "0000000000000000000000000000000000000000000000000000000000000009"
    ]);
    pub static ref IS_LESS_ADDRESS: AccountId = to_account_from_seed(&hex![
        "0000000000000000000000000000000000000000000000000000000000000010"
    ]);
    pub static ref IS_STORED_ADDRESS: AccountId = to_account_from_seed(&hex![
        "0000000000000000000000000000000000000000000000000000000000000011"
    ]);
    pub static ref IS_VALID_SIGNATURE_ADDRESS: AccountId = to_account_from_seed(&hex![
        "0000000000000000000000000000000000000000000000000000000000000012"
    ]);
    pub static ref VERIFY_INCLUSION_ADDRESS: AccountId = to_account_from_seed(&hex![
        "0000000000000000000000000000000000000000000000000000000000000013"
    ]);
    pub static ref SECP_256_K1: H256 = H256::from(&hex![
        "d4fa99b1e08c4e5e6deb461846aa629344d95ff03ed04754c2053d54c756f439"
    ]);
}

struct MockAtomicPredicateIdConfigGetter;
impl Get<AtomicPredicateIdConfig<AccountId, H256>> for MockAtomicPredicateIdConfigGetter {
    fn get() -> AtomicPredicateIdConfig<AccountId, H256> {
        AtomicPredicateIdConfig {
            not_address: (*NOT_ADDRESS).clone(),
            and_address: (*AND_ADDRESS).clone(),
            or_address: (*OR_ADDRESS).clone(),
            for_all_address: (*FOR_ALL_ADDRESS).clone(),
            there_exists_address: (*THERE_EXISTS_ADDRESS).clone(),
            equal_address: (*EQUAL_ADDRESS).clone(),
            is_contained_address: (*IS_CONTAINED_ADDRESS).clone(),
            is_less_address: (*IS_LESS_ADDRESS).clone(),
            is_stored_address: (*IS_STORED_ADDRESS).clone(),
            is_valid_signature_address: (*IS_VALID_SIGNATURE_ADDRESS).clone(),
            verify_inclusion_address: (*VERIFY_INCLUSION_ADDRESS).clone(),
            secp256k1: (*SECP_256_K1).clone(),
        }
    }
}

impl pallet_ovm::Trait for Test {
    type MaxDepth = MaxDepth;
    type DisputePeriod = DisputePeriod;
    type DeterminePredicateAddress = ovm::SimpleAddressDeterminer<Test>;
    type HashingL2 = BlakeTwo256;
    type ExternalCall = ovm::predicate::CallContext<Test>;
    type AtomicPredicateIdConfig = MockAtomicPredicateIdConfigGetter;
    type Event = MetaEvent;
}

parameter_types! {
    pub const MaximumTokenAddress: AccountId = AccountId::max_value();
}

impl Trait for Test {
    type Currency = Balances;
    type DeterminePlappsAddress = SimpleAddressDeterminer<T>;
    type MaximumTokenAddress = MaximumTokenAddress;
    // TODO: should be Keccak;
    type PlasmaHashing = BlakeTwo256;
    type Event = MetaEvent;
}

pub type System = frame_system::Module<Test>;
pub type Balances = pallet_balances::Module<Test>;
pub type Contracts = pallet_contracts::Module<Test>;
// pub type Ovm = pallet_ovm::Module<Test>;
pub type Plasma = Module<Test>;
pub type Timestamp = pallet_timestamp::Module<Test>;
pub type Randomness = pallet_randomness_collective_flip::Module<Test>;

pub fn advance_block() {
    System::finalize();
    let next = System::block_number() + 1;
    // increase block numebr
    System::set_block_number(next);
    System::initialize(
        &next,
        &[0u8; 32].into(),
        &[0u8; 32].into(),
        &Default::default(),
        system::InitKind::Full,
    );
    System::note_finished_initialize();
}

/// Generate compiled predicate binary and code hash from predicate source.
pub fn compile_predicate<T>(predicate_module: &str) -> (Vec<u8>, <T::Hashing as Hash>::Output)
where
    T: frame_system::Trait,
{
    // TODO actually predicate to compiled predicate.
    let compiled_predicate = predicate_module.as_bytes().to_vec();
    let code_hash = T::Hashing::hash_of(&compiled_predicate);
    (compiled_predicate.to_vec(), code_hash)
}

pub fn to_account_from_seed(seed: &[u8; 32]) -> AccountId {
    to_account(sp_core::ecdsa::Pair::from_seed(&seed).public().as_ref())
}

pub fn to_account(full_public: &[u8]) -> AccountId {
    let public = sp_core::ecdsa::Public::from_full(full_public).unwrap();
    sp_runtime::MultiSigner::from(public).into_account()
}
