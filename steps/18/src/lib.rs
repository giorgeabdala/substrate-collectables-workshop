#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// Learn about Macros used in the `polkadot-sdk`, making pallet development easier.
#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	// Learn about the Pallet struct: the structure on which we implement all functions and traits
	// for the Pallet.
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// Learn about frame_system, and `Config`.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	/// Learn about storage value.
	#[pallet::storage]
	pub(super) type CountForKitties<T: Config> = StorageValue<Value = u64, QueryKind = ValueQuery>;

	/// Learn about storage maps.
	#[pallet::storage]
	pub(super) type Kitties<T: Config> = StorageMap<Key = [u8; 16], Value = ()>;

	// Learn about events.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created { owner: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		TooManyKitties,
		DuplicateKitty,
	}

	// Learn about callable functions and dispatch.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
			// Learn about `origin`.
			let who = ensure_signed(origin)?;
			let dna = [0u8; 16];
			Self::mint(who, dna)?;
			Ok(())
		}
	}

	// Learn about internal functions.
	impl<T: Config> Pallet<T> {
		// Learn about `AccountId`.
		fn mint(owner: T::AccountId, dna: [u8; 16]) -> DispatchResult {
			// Check if the kitty does not already exist in our storage map
			ensure!(!Kitties::<T>::contains_key(dna), Error::<T>::DuplicateKitty);

			let current_count: u64 = CountForKitties::<T>::get();
			let new_count = current_count.checked_add(1).ok_or(Error::<T>::TooManyKitties)?;
			Kitties::<T>::insert(dna, ());
			CountForKitties::<T>::set(new_count);
			Self::deposit_event(Event::<T>::Created { owner });
			Ok(())
		}
	}
}