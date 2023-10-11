
//! Autogenerated weights for pallet_file_bank
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-04, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ubuntu`, CPU: `Intel(R) Core(TM) i5-10400 CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("cess-initial-testnet"), DB CACHE: 1024

// Executed Command:
// ./target/release/cess-node
// benchmark
// pallet
// --chain
// cess-initial-testnet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_file_bank
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template=./.maintain/frame-weight-template.hbs
// --output=./c-pallets/file-bank/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_file_bank.
pub trait WeightInfo {
	fn cert_idle_space() -> Weight;
	fn upload_declaration(v: u32, ) -> Weight;
	fn upload_declaration_expected_max(v: u32, ) -> Weight;
	fn transfer_report(v: u32, ) -> Weight;
	fn transfer_report_last(v: u32, ) -> Weight;
	fn upload_declaration_fly_upload(v: u32, ) -> Weight;
	fn deal_reassign_miner(v: u32, ) -> Weight;
	fn deal_reassign_miner_exceed_limit(v: u32, ) -> Weight;
	fn calculate_end(v: u32, ) -> Weight;
	fn replace_idle_space() -> Weight;
	fn delete_file(v: u32, ) -> Weight;
	fn create_bucket() -> Weight;
	fn delete_bucket() -> Weight;
	fn generate_restoral_order() -> Weight;
	fn claim_restoral_order() -> Weight;
	fn restoral_order_complete() -> Weight;
	fn claim_restoral_noexist_order() -> Weight;
}

/// Weights for pallet_file_bank using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: TeeWorker TeePodr2Pk (r:1 w:0)
	// Storage: Sminer MinerItems (r:1 w:1)
	// Storage: StorageHandler TotalIdleSpace (r:1 w:1)
	fn cert_idle_space() -> Weight {
		// Minimum execution time: 1_465_084 nanoseconds.
		Weight::from_parts(1_520_990_000, 0)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: StorageHandler UserOwnedSpace (r:1 w:1)
	// Storage: FileBank File (r:1 w:0)
	// Storage: Sminer AllMiner (r:1 w:0)
	// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	// Storage: FileBank TaskFailedCount (r:1 w:0)
	// Storage: Sminer MinerItems (r:1 w:1)
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: FileBank DealMap (r:0 w:1)
	/// The range of component `v` is `[1, 30]`.
	fn upload_declaration(v: u32, ) -> Weight {
		// Minimum execution time: 130_096 nanoseconds.
		Weight::from_parts(173_201_944, 0)
			// Standard Error: 112_849
			.saturating_add(Weight::from_parts(3_023_108, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: StorageHandler UserOwnedSpace (r:1 w:1)
	// Storage: FileBank File (r:1 w:0)
	// Storage: Sminer AllMiner (r:1 w:0)
	// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	// Storage: FileBank TaskFailedCount (r:15 w:0)
	// Storage: Sminer MinerItems (r:15 w:2)
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: FileBank DealMap (r:0 w:1)
	/// The range of component `v` is `[1, 30]`.
	fn upload_declaration_expected_max(_v: u32, ) -> Weight {
		// Minimum execution time: 372_440 nanoseconds.
		Weight::from_parts(472_294_231, 0)
			.saturating_add(T::DbWeight::get().reads(36))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: FileBank DealMap (r:1 w:1)
	/// The range of component `v` is `[1, 30]`.
	fn transfer_report(v: u32, ) -> Weight {
		// Minimum execution time: 57_255 nanoseconds.
		Weight::from_parts(68_385_886, 0)
			// Standard Error: 37_035
			.saturating_add(Weight::from_parts(519_563, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: FileBank DealMap (r:1 w:1)
	// Storage: FileBank PendingReplacements (r:3 w:3)
	// Storage: FileBank TaskFailedCount (r:3 w:3)
	// Storage: StorageHandler UserOwnedSpace (r:1 w:1)
	// Storage: StorageHandler TotalIdleSpace (r:1 w:1)
	// Storage: StorageHandler TotalServiceSpace (r:1 w:1)
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: FileBank Bucket (r:1 w:1)
	// Storage: FileBank UserBucketList (r:1 w:1)
	// Storage: FileBank UserHoldFileList (r:1 w:1)
	// Storage: FileBank File (r:0 w:1)
	/// The range of component `v` is `[1, 30]`.
	fn transfer_report_last(v: u32, ) -> Weight {
		// Minimum execution time: 177_496 nanoseconds.
		Weight::from_parts(180_547_407, 0)
			// Standard Error: 74_093
			.saturating_add(Weight::from_parts(4_661_110, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(16))
			.saturating_add(T::DbWeight::get().writes(17))
	}
	// Storage: StorageHandler UserOwnedSpace (r:1 w:1)
	// Storage: FileBank File (r:1 w:1)
	// Storage: FileBank Bucket (r:1 w:1)
	// Storage: FileBank UserBucketList (r:1 w:1)
	// Storage: FileBank UserHoldFileList (r:1 w:1)
	/// The range of component `v` is `[1, 30]`.
	fn upload_declaration_fly_upload(v: u32, ) -> Weight {
		// Minimum execution time: 100_259 nanoseconds.
		Weight::from_parts(115_372_577, 0)
			// Standard Error: 51_183
			.saturating_add(Weight::from_parts(943_540, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: FileBank DealMap (r:1 w:1)
	// Storage: Sminer AllMiner (r:1 w:0)
	// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	// Storage: FileBank TaskFailedCount (r:18 w:3)
	// Storage: Sminer MinerItems (r:18 w:6)
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `v` is `[0, 30]`.
	fn deal_reassign_miner(v: u32, ) -> Weight {
		// Minimum execution time: 381_575 nanoseconds.
		Weight::from_parts(411_033_067, 0)
			// Standard Error: 74_850
			.saturating_add(Weight::from_parts(1_245_688, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(41))
			.saturating_add(T::DbWeight::get().writes(12))
	}
	// Storage: FileBank DealMap (r:1 w:1)
	// Storage: StorageHandler UserOwnedSpace (r:1 w:1)
	// Storage: Sminer MinerItems (r:3 w:3)
	/// The range of component `v` is `[0, 30]`.
	fn deal_reassign_miner_exceed_limit(v: u32, ) -> Weight {
		// Minimum execution time: 99_424 nanoseconds.
		Weight::from_parts(111_303_061, 0)
			// Standard Error: 33_521
			.saturating_add(Weight::from_parts(586_379, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: FileBank DealMap (r:1 w:1)
	// Storage: Sminer MinerItems (r:3 w:3)
	// Storage: FileBank File (r:1 w:1)
	/// The range of component `v` is `[1, 30]`.
	fn calculate_end(v: u32, ) -> Weight {
		// Minimum execution time: 144_573 nanoseconds.
		Weight::from_parts(151_655_693, 0)
			// Standard Error: 38_575
			.saturating_add(Weight::from_parts(3_286_351, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: TeeWorker TeePodr2Pk (r:1 w:0)
	// Storage: Sminer MinerItems (r:1 w:1)
	// Storage: FileBank PendingReplacements (r:1 w:1)
	fn replace_idle_space() -> Weight {
		// Minimum execution time: 1_484_864 nanoseconds.
		Weight::from_parts(1_519_692_000, 0)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: FileBank File (r:1 w:1)
	// Storage: StorageHandler UserOwnedSpace (r:1 w:1)
	// Storage: StorageHandler TotalServiceSpace (r:1 w:1)
	// Storage: FileBank Bucket (r:1 w:1)
	// Storage: FileBank UserHoldFileList (r:1 w:1)
	// Storage: Sminer RestoralTarget (r:3 w:0)
	// Storage: Sminer MinerItems (r:3 w:3)
	/// The range of component `v` is `[0, 30]`.
	fn delete_file(v: u32, ) -> Weight {
		// Minimum execution time: 87_256 nanoseconds.
		Weight::from_parts(195_439_527, 0)
			// Standard Error: 121_694
			.saturating_add(Weight::from_parts(4_058_900, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	// Storage: FileBank Bucket (r:1 w:1)
	// Storage: FileBank UserBucketList (r:1 w:1)
	fn create_bucket() -> Weight {
		// Minimum execution time: 44_383 nanoseconds.
		Weight::from_parts(45_277_000, 0)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: FileBank Bucket (r:1 w:1)
	// Storage: FileBank UserBucketList (r:1 w:1)
	fn delete_bucket() -> Weight {
		// Minimum execution time: 52_452 nanoseconds.
		Weight::from_parts(55_820_000, 0)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: FileBank RestoralOrder (r:1 w:1)
	// Storage: FileBank File (r:1 w:1)
	fn generate_restoral_order() -> Weight {
		// Minimum execution time: 88_578 nanoseconds.
		Weight::from_parts(94_129_000, 0)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Sminer MinerItems (r:1 w:0)
	// Storage: FileBank RestoralOrder (r:1 w:1)
	fn claim_restoral_order() -> Weight {
		// Minimum execution time: 64_488 nanoseconds.
		Weight::from_parts(70_910_000, 0)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Sminer MinerItems (r:2 w:2)
	// Storage: FileBank RestoralOrder (r:1 w:1)
	// Storage: FileBank File (r:1 w:1)
	// Storage: FileBank PendingReplacements (r:1 w:1)
	// Storage: Sminer RestoralTarget (r:1 w:0)
	fn restoral_order_complete() -> Weight {
		// Minimum execution time: 185_004 nanoseconds.
		Weight::from_parts(196_225_000, 0)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: Sminer MinerItems (r:1 w:0)
	// Storage: FileBank RestoralOrder (r:1 w:1)
	// Storage: Sminer RestoralTarget (r:1 w:0)
	// Storage: FileBank File (r:1 w:1)
	fn claim_restoral_noexist_order() -> Weight {
		// Minimum execution time: 106_741 nanoseconds.
		Weight::from_parts(113_477_000, 0)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: TeeWorker TeePodr2Pk (r:1 w:0)
	// Storage: Sminer MinerItems (r:1 w:1)
	// Storage: StorageHandler TotalIdleSpace (r:1 w:1)
	fn cert_idle_space() -> Weight {
		// Minimum execution time: 1_465_084 nanoseconds.
		Weight::from_parts(1_520_990_000, 0)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: StorageHandler UserOwnedSpace (r:1 w:1)
	// Storage: FileBank File (r:1 w:0)
	// Storage: Sminer AllMiner (r:1 w:0)
	// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	// Storage: FileBank TaskFailedCount (r:1 w:0)
	// Storage: Sminer MinerItems (r:1 w:1)
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: FileBank DealMap (r:0 w:1)
	/// The range of component `v` is `[1, 30]`.
	fn upload_declaration(v: u32, ) -> Weight {
		// Minimum execution time: 130_096 nanoseconds.
		Weight::from_parts(173_201_944, 0)
			// Standard Error: 112_849
			.saturating_add(Weight::from_parts(3_023_108, 0).saturating_mul(v.into()))
			.saturating_add(RocksDbWeight::get().reads(11))
			.saturating_add(RocksDbWeight::get().writes(7))
	}
	// Storage: StorageHandler UserOwnedSpace (r:1 w:1)
	// Storage: FileBank File (r:1 w:0)
	// Storage: Sminer AllMiner (r:1 w:0)
	// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	// Storage: FileBank TaskFailedCount (r:15 w:0)
	// Storage: Sminer MinerItems (r:15 w:2)
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: FileBank DealMap (r:0 w:1)
	/// The range of component `v` is `[1, 30]`.
	fn upload_declaration_expected_max(_v: u32, ) -> Weight {
		// Minimum execution time: 372_440 nanoseconds.
		Weight::from_parts(472_294_231, 0)
			.saturating_add(RocksDbWeight::get().reads(36))
			.saturating_add(RocksDbWeight::get().writes(6))
	}
	// Storage: FileBank DealMap (r:1 w:1)
	/// The range of component `v` is `[1, 30]`.
	fn transfer_report(v: u32, ) -> Weight {
		// Minimum execution time: 57_255 nanoseconds.
		Weight::from_parts(68_385_886, 0)
			// Standard Error: 37_035
			.saturating_add(Weight::from_parts(519_563, 0).saturating_mul(v.into()))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: FileBank DealMap (r:1 w:1)
	// Storage: FileBank PendingReplacements (r:3 w:3)
	// Storage: FileBank TaskFailedCount (r:3 w:3)
	// Storage: StorageHandler UserOwnedSpace (r:1 w:1)
	// Storage: StorageHandler TotalIdleSpace (r:1 w:1)
	// Storage: StorageHandler TotalServiceSpace (r:1 w:1)
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: FileBank Bucket (r:1 w:1)
	// Storage: FileBank UserBucketList (r:1 w:1)
	// Storage: FileBank UserHoldFileList (r:1 w:1)
	// Storage: FileBank File (r:0 w:1)
	/// The range of component `v` is `[1, 30]`.
	fn transfer_report_last(v: u32, ) -> Weight {
		// Minimum execution time: 177_496 nanoseconds.
		Weight::from_parts(180_547_407, 0)
			// Standard Error: 74_093
			.saturating_add(Weight::from_parts(4_661_110, 0).saturating_mul(v.into()))
			.saturating_add(RocksDbWeight::get().reads(16))
			.saturating_add(RocksDbWeight::get().writes(17))
	}
	// Storage: StorageHandler UserOwnedSpace (r:1 w:1)
	// Storage: FileBank File (r:1 w:1)
	// Storage: FileBank Bucket (r:1 w:1)
	// Storage: FileBank UserBucketList (r:1 w:1)
	// Storage: FileBank UserHoldFileList (r:1 w:1)
	/// The range of component `v` is `[1, 30]`.
	fn upload_declaration_fly_upload(v: u32, ) -> Weight {
		// Minimum execution time: 100_259 nanoseconds.
		Weight::from_parts(115_372_577, 0)
			// Standard Error: 51_183
			.saturating_add(Weight::from_parts(943_540, 0).saturating_mul(v.into()))
			.saturating_add(RocksDbWeight::get().reads(5))
			.saturating_add(RocksDbWeight::get().writes(5))
	}
	// Storage: FileBank DealMap (r:1 w:1)
	// Storage: Sminer AllMiner (r:1 w:0)
	// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	// Storage: FileBank TaskFailedCount (r:18 w:3)
	// Storage: Sminer MinerItems (r:18 w:6)
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `v` is `[0, 30]`.
	fn deal_reassign_miner(v: u32, ) -> Weight {
		// Minimum execution time: 381_575 nanoseconds.
		Weight::from_parts(411_033_067, 0)
			// Standard Error: 74_850
			.saturating_add(Weight::from_parts(1_245_688, 0).saturating_mul(v.into()))
			.saturating_add(RocksDbWeight::get().reads(41))
			.saturating_add(RocksDbWeight::get().writes(12))
	}
	// Storage: FileBank DealMap (r:1 w:1)
	// Storage: StorageHandler UserOwnedSpace (r:1 w:1)
	// Storage: Sminer MinerItems (r:3 w:3)
	/// The range of component `v` is `[0, 30]`.
	fn deal_reassign_miner_exceed_limit(v: u32, ) -> Weight {
		// Minimum execution time: 99_424 nanoseconds.
		Weight::from_parts(111_303_061, 0)
			// Standard Error: 33_521
			.saturating_add(Weight::from_parts(586_379, 0).saturating_mul(v.into()))
			.saturating_add(RocksDbWeight::get().reads(5))
			.saturating_add(RocksDbWeight::get().writes(5))
	}
	// Storage: FileBank DealMap (r:1 w:1)
	// Storage: Sminer MinerItems (r:3 w:3)
	// Storage: FileBank File (r:1 w:1)
	/// The range of component `v` is `[1, 30]`.
	fn calculate_end(v: u32, ) -> Weight {
		// Minimum execution time: 144_573 nanoseconds.
		Weight::from_parts(151_655_693, 0)
			// Standard Error: 38_575
			.saturating_add(Weight::from_parts(3_286_351, 0).saturating_mul(v.into()))
			.saturating_add(RocksDbWeight::get().reads(5))
			.saturating_add(RocksDbWeight::get().writes(5))
	}
	// Storage: TeeWorker TeePodr2Pk (r:1 w:0)
	// Storage: Sminer MinerItems (r:1 w:1)
	// Storage: FileBank PendingReplacements (r:1 w:1)
	fn replace_idle_space() -> Weight {
		// Minimum execution time: 1_484_864 nanoseconds.
		Weight::from_parts(1_519_692_000, 0)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: FileBank File (r:1 w:1)
	// Storage: StorageHandler UserOwnedSpace (r:1 w:1)
	// Storage: StorageHandler TotalServiceSpace (r:1 w:1)
	// Storage: FileBank Bucket (r:1 w:1)
	// Storage: FileBank UserHoldFileList (r:1 w:1)
	// Storage: Sminer RestoralTarget (r:3 w:0)
	// Storage: Sminer MinerItems (r:3 w:3)
	/// The range of component `v` is `[0, 30]`.
	fn delete_file(v: u32, ) -> Weight {
		// Minimum execution time: 87_256 nanoseconds.
		Weight::from_parts(195_439_527, 0)
			// Standard Error: 121_694
			.saturating_add(Weight::from_parts(4_058_900, 0).saturating_mul(v.into()))
			.saturating_add(RocksDbWeight::get().reads(10))
			.saturating_add(RocksDbWeight::get().writes(8))
	}
	// Storage: FileBank Bucket (r:1 w:1)
	// Storage: FileBank UserBucketList (r:1 w:1)
	fn create_bucket() -> Weight {
		// Minimum execution time: 44_383 nanoseconds.
		Weight::from_parts(45_277_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: FileBank Bucket (r:1 w:1)
	// Storage: FileBank UserBucketList (r:1 w:1)
	fn delete_bucket() -> Weight {
		// Minimum execution time: 52_452 nanoseconds.
		Weight::from_parts(55_820_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: FileBank RestoralOrder (r:1 w:1)
	// Storage: FileBank File (r:1 w:1)
	fn generate_restoral_order() -> Weight {
		// Minimum execution time: 88_578 nanoseconds.
		Weight::from_parts(94_129_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: Sminer MinerItems (r:1 w:0)
	// Storage: FileBank RestoralOrder (r:1 w:1)
	fn claim_restoral_order() -> Weight {
		// Minimum execution time: 64_488 nanoseconds.
		Weight::from_parts(70_910_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Sminer MinerItems (r:2 w:2)
	// Storage: FileBank RestoralOrder (r:1 w:1)
	// Storage: FileBank File (r:1 w:1)
	// Storage: FileBank PendingReplacements (r:1 w:1)
	// Storage: Sminer RestoralTarget (r:1 w:0)
	fn restoral_order_complete() -> Weight {
		// Minimum execution time: 185_004 nanoseconds.
		Weight::from_parts(196_225_000, 0)
			.saturating_add(RocksDbWeight::get().reads(6))
			.saturating_add(RocksDbWeight::get().writes(5))
	}
	// Storage: Sminer MinerItems (r:1 w:0)
	// Storage: FileBank RestoralOrder (r:1 w:1)
	// Storage: Sminer RestoralTarget (r:1 w:0)
	// Storage: FileBank File (r:1 w:1)
	fn claim_restoral_noexist_order() -> Weight {
		// Minimum execution time: 106_741 nanoseconds.
		Weight::from_parts(113_477_000, 0)
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
}
