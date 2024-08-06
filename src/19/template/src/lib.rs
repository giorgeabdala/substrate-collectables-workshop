#![cfg_attr(not(feature = "std"), no_std)]

mod impls;

use frame::prelude::*;
pub use pallet::*;

#[frame::pallet(dev_mode)]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	/* 🚧 TODO 🚧:
		- Add the derive macros needed for putting a struct in storage.
		- Add `#[scale_info(skip_type_params(T))]` to ignore the generic `T`.
	*/
	pub struct Kitty<T: Config> {
		// Using 32 bytes to represent a kitty DNA
		pub dna: [u8; 32],
		pub owner: T::AccountId,
	}

	#[pallet::storage]
	pub(super) type CountForKitties<T: Config> = StorageValue<Value = u32, QueryKind = ValueQuery>;

	#[pallet::storage]
	/* 🚧 TODO 🚧: Update the `Value` to be type `Kitty<T>` instead of (). */
	pub(super) type Kitties<T: Config> = StorageMap<Key = [u8; 32], Value = ()>;

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

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let dna = [0u8; 32];
			Self::mint(who, dna)?;
			Ok(())
		}
	}
}
