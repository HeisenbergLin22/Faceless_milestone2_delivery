use crate as pallet_faceless;
use frame_support::traits::{ConstU16, ConstU64};
use frame_support::{parameter_types};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Config<T>, Storage, Event<T>},
		FacelessModule: pallet_faceless::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub static ExistentialDeposit: u64 = 1;
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
	type AccountData = pallet_balances::AccountData<u64>;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = u64;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

impl pallet_faceless::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	// let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap().into();
	// pallet_balances::GenesisConfig::<Test> {
	// 	balances: vec![
	// 	 (1, 10),
	// 	 (2, 20),
	// 	 (3, 30),
	// 	 (4, 40),
	// 	 (5, 50),
	// 	 (6, 60)
	// 	],
	//    }
	// 	.assimilate_storage(&mut t)
	// 	.unwrap();
	// t
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
