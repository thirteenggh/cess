//! # Segemnt Book Module
//!
//! Contain operations related proof of storage.
//!
//! ### Terminology
//! 
//! * **uncid:** 		Necessary parameters for generating proof (unencrypted)
//! * **sealed_cid:** 	Necessary parameters for generating proof (encrypted)
//! * **segment_id:**	Allocated segment ID
//! * **is_ready:**		Used to know whether to submit a certificate
//! * **size_type:**	Segment size
//! * **peer_id:**		Miner's ID 
//! 
//! ### Interface
//!
//! ### Dispatchable Functions
//!
//! * `intent_submit` 		Pprovide miners with the necessary parameters to generate proof
//! * `intent_submit_po_st` Provide miners with the necessary parameters to generate proof
//! * `submit_to_vpa` 		Submit copy certificate of idle data segment
//! * `verify_in_vpa` 		Verify replication proof of idle data segments
//! * `submit_to_vpb` 		Submit space-time proof of idle data segments
//! * `verify_in_vpb` 		Verify the spatiotemporal proof of idle data segments
//! * `submit_to_vpc` 		Submit a copy certificate of the service data segment
//! * `verify_in_vpc` 		Verify the replication certificate of the service data segment
//! * `submit_to_vpd` 		Submit spatio-temporal proof of service data segment
//! * `verify_in_vpd` 		Verify the spatio-temporal proof of service data segments


#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::{ReservableCurrency};
pub use pallet::*;
use scale_info::TypeInfo;
use sp_runtime::{
	RuntimeDebug,
    traits::SaturatedConversion,
};

use codec::{Encode, Decode};
use frame_support::{dispatch::DispatchResult, PalletId};
use sp_std::prelude::*;

type AccountOf<T> = <T as frame_system::Config>::AccountId;
type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;


#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct SchedulerInfo<T: pallet::Config> {
    ip: Vec<u8>,
    stash_user: AccountOf<T>,
    controller_user: AccountOf<T>,
}

#[derive(PartialEq, Eq, Encode, Decode, Clone, RuntimeDebug, Default, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ExceptionReport<T: pallet::Config> {
    count: u32,
    reporter: Vec<AccountOf<T>>,
} 

#[frame_support::pallet]
pub mod pallet {
    use super::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{Get}, Blake2_128Concat
	};
	use frame_system::{ensure_signed, pallet_prelude::*};


	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_cess_staking::Config + std::default::Default {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// The currency trait.
		type Currency: ReservableCurrency<Self::AccountId>;
		/// pallet address.
		#[pallet::constant]
		type FileMapPalletId: Get<PalletId>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
        //Scheduling registration method
        RegistrationScheduler{acc: AccountOf<T>, ip: Vec<u8>},
    }

    #[pallet::error]
    pub enum Error<T> {
        //Already registered
        AlreadyRegistration,

        NotController,

        AlreadyReport,
    }

    #[pallet::storage]
    #[pallet::getter(fn scheduler_map)]
    pub(super) type SchedulerMap<T: Config> = StorageValue<_, Vec<SchedulerInfo<T>>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn scheduler_exception)]
    pub(super) type SchedulerException<T: Config> = StorageMap<_, Blake2_128Concat, AccountOf<T>, ExceptionReport<T>, ValueQuery>;

    #[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

    #[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberOf<T>> for Pallet<T> {
		//Used to calculate whether it is implied to submit spatiotemporal proof
		//Cycle every 7.2 hours
		//When there is an uncommitted space-time certificate, the corresponding miner will be punished 
		//and the corresponding data segment will be removed
		fn on_initialize(now: BlockNumberOf<T>) -> Weight {
			let number: u128 = now.saturated_into();
			let count: usize = Self::scheduler_map().len();
			if number % 1200 == 0 {
				for (key ,value) in <SchedulerException<T>>::iter() {
                    if value.count > ( count / 2 ) as u32 {

                    }

                    <SchedulerException<T>>::remove(key);
                }
			}
			0
		}
	}

    #[pallet::call]
	impl<T: Config> Pallet<T> {
        //Scheduling registration method
        #[pallet::weight(1_000_000)]
        pub fn registration_scheduler(origin: OriginFor<T>, stash_account: AccountOf<T>, ip: Vec<u8>) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let acc = <pallet_cess_staking::Pallet<T>>::bonded(&stash_account).unwrap();
            if sender != acc {
                Err(Error::<T>::NotController)?;
            }
            let mut s_vec = SchedulerMap::<T>::get();
            let scheduler = SchedulerInfo::<T>{
                ip: ip.clone(),
                stash_user: stash_account.clone(),
                controller_user: sender.clone(),
            };

            if s_vec.contains(&scheduler) {
                Err(Error::<T>::AlreadyRegistration)?;
            }
            s_vec.push(scheduler);
            SchedulerMap::<T>::put(s_vec);
            Self::deposit_event(Event::<T>::RegistrationScheduler{acc: sender, ip: ip});
            Ok(())
        }

        #[pallet::weight(1_000_000)]
        pub fn scheduler_exception_report(origin: OriginFor<T>, account: AccountOf<T>) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            <SchedulerException<T>>::try_mutate(&account, |o| -> DispatchResult {
                for value in &o.reporter {
                    if &sender == value {
                        Err(Error::<T>::AlreadyReport)?;
                    }
                }
                o.count += 1;
                o.reporter.push(account.clone());
                Ok(())
            })?; 

            Ok(())
        }
    }
}