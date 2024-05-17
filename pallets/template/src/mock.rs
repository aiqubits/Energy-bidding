use crate as energy_bidding;
use frame_support::traits::{ConstU16, ConstU64};
use sp_core::H256;
use sp_runtime::{
    BuildStorage,
    traits::{BlakeTwo256, IdentityLookup}
};

// type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    MultiSignature,
};

pub type Signature = MultiSignature;

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		EnergyBiddingModule: energy_bidding,
	}
);

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
	type Nonce = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

// impl frame_system::Config for Test {
// 	type BaseCallFilter = frame_support::traits::Everything;
// 	type BlockWeights = ();
// 	type BlockLength = ();
// 	type DbWeight = ();
// 	type RuntimeOrigin = RuntimeOrigin;
// 	type RuntimeCall = RuntimeCall;
// 	type Nonce = u64;
// 	type Hash = H256;
// 	type Hashing = BlakeTwo256;
// 	type AccountId = u64;
// 	type Lookup = IdentityLookup<Self::AccountId>;
// 	type Block = Block;
// 	type RuntimeEvent = RuntimeEvent;
// 	type BlockHashCount = ConstU64<250>;
// 	type Version = ();
// 	type PalletInfo = PalletInfo;
// 	type AccountData = ();
// 	type OnNewAccount = ();
// 	type OnKilledAccount = ();
// 	type SystemWeightInfo = ();
// 	type SS58Prefix = ConstU16<42>;
// 	type OnSetCode = ();
// 	type MaxConsumers = frame_support::traits::ConstU32<16>;
// }

impl energy_bidding::Config for Test {
	type RuntimeEvent = RuntimeEvent;
    type AuctionId = u64;
    type Quantity = u128;
    type Price = u128;
	// type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
