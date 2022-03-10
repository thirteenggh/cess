use super::*;

type AccountOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;

#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct FileInfo<T: pallet::Config> {
	pub(super) file_name: Vec<u8>,
	pub(super) file_size: u128,
	pub(super) file_hash: Vec<u8>,
	//Public or not
	pub(super) public: bool,
	pub(super) user_addr: AccountOf<T>,
	//normal or repairing
	pub(super) file_state: Vec<u8>,
	//Number of backups
	pub(super) backups: u8,
	pub(super) downloadfee: BalanceOf<T>,
	//Backup information
	pub(super) file_dupl: Vec<FileDuplicateInfo<T>>,
}

//backups info struct
#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct FileDuplicateInfo<T: pallet::Config> {
	pub(super) dupl_id: Vec<u8>,
	pub(super) rand_key: Vec<u8>,
	pub(super) slice_num: u16,
	pub(super) file_slice: Vec<FileSliceInfo<T>>,
}

//slice info
//Slice consists of shard
#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct FileSliceInfo<T: pallet::Config> {
	pub(super) slice_id: Vec<u8>,
	pub(super) slice_size: u16,
	pub(super) slice_hash: Vec<u8>,
	pub(super) file_shard: FileShardInfo<T>,
}

//shard info
//Slice consists of shard
#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct FileShardInfo<T: pallet::Config> {
	pub(super) data_shard_num: u8,
	pub(super) redun_shard_num: u8,
	pub(super) shard_hash: Vec<Vec<u8>>,
	pub(super) shard_addr: Vec<Vec<u8>>,
	pub(super) wallet_addr: Vec<AccountOf<T>>,
}

#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
pub struct StorageSpace {
	pub(super) purchased_space: u128,
	pub(super) used_space: u128,
	pub(super) remaining_space: u128,
}

#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct SpaceInfo<T: pallet::Config> {
	pub(super) size: u128,
	pub(super) deadline: BlockNumberOf<T>,
}

#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
pub struct FileSlice {
	pub(super) peer_id: Vec<Vec<u8>>,
	pub(super) fill_zero: u32,
	pub(super) slice_id: u64,
}