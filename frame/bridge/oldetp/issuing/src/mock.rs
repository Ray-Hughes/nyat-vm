// This file is part of Hyperspace.
//
// Copyright (C) 2018-2021 Hyperspace Network
// SPDX-License-Identifier: GPL-3.0
//
// Hyperspace is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Hyperspace is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Hyperspace. If not, see <https://www.gnu.org/licenses/>.

#![allow(dead_code)]

// --- crates ---
use codec::{Decode, Encode};
// --- substrate ---
use frame_system::mocking::*;
use sp_io::TestExternalities;
use sp_runtime::{
	testing::{Header, H256},
	traits::{BlakeTwo256, IdentityLookup},
	RuntimeDebug,
};
// --- hyperspace ---
use crate::{self as hyperspace_oldetp_issuing, *};

pub type Block = MockBlock<Test>;
pub type UncheckedExtrinsic = MockUncheckedExtrinsic<Test>;

pub type AccountId = u64;
pub type Balance = u128;

pub type OldetpIssuingError = Error<Test>;

hyperspace_support::impl_test_account_data! {}

impl frame_system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

frame_support::parameter_types! {
	pub const ExistentialDeposit: Balance = 0;
}
impl hyperspace_balances::Config<EtpInstance> for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type BalanceInfo = AccountData<Balance>;
	type AccountStore = System;
	type MaxLocks = ();
	type OtherCurrencies = ();
	type WeightInfo = ();
}

frame_support::parameter_types! {
	pub const OldetpIssuingModuleId: ModuleId = ModuleId(*b"da/oldetpi");
}
impl Config for Test {
	type Event = Event;
	type ModuleId = OldetpIssuingModuleId;
	type EtpCurrency = Etp;
	type WeightInfo = ();
}

frame_support::construct_runtime! {
	pub enum Test
	where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Module, Call, Storage, Config, Event<T>},
		Etp: hyperspace_balances::<Instance0>::{Module, Call, Storage, Config<T>, Event<T>},
		OldetpIssuing: hyperspace_oldetp_issuing::{Module, Call, Storage, Config, Event<T>},
	}
}

pub fn new_test_ext() -> TestExternalities {
	let mut t = frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap();

	hyperspace_balances::GenesisConfig::<Test, EtpInstance> {
		balances: (1..10)
			.map(|i: AccountId| vec![(i, 100 * i as Balance), (10 * i, 1000 * i as Balance)])
			.flatten()
			.collect(),
	}
	.assimilate_storage(&mut t)
	.unwrap();
	hyperspace_oldetp_issuing::GenesisConfig {
		total_mapped_etp: 4_000,
	}
	.assimilate_storage::<Test>(&mut t)
	.unwrap();

	t.into()
}

pub fn events() -> Vec<Event> {
	let events = System::events()
		.into_iter()
		.map(|evt| evt.event)
		.collect::<Vec<_>>();

	System::reset_events();

	events
}

pub fn oldetp_issuing_events() -> Vec<Event> {
	events()
		.into_iter()
		.filter(|e| matches!(e, Event::hyperspace_oldetp_issuing(_)))
		.collect()
}
