#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
use frame_support::{
	debug, decl_error, decl_event, decl_module, decl_storage, dispatch,
	traits::{Currency, ReservableCurrency},
};
use frame_system::ensure_signed;
use sp_std::vec::Vec;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(Encode, Decode, Default)]
pub struct TaskDetails<AccountId, Balance> {
	client: AccountId,
	worker_id: Option<AccountId>,
	dur: u64,
	des: Vec<u8>,
	cost: Balance,
}

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	type Currency: ReservableCurrency<Self::AccountId>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Config> as TemplateModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items

			TaskStorage get(fn task):
			map hasher(blake2_128_concat) Vec<u8> => TaskDetails<T::AccountId, BalanceOf<T>>;
			AccountBalances get(fn get_account_balances):
			map hasher(blake2_128_concat) T::AccountId => BalanceOf<T>;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
		Balance = BalanceOf<T>,
	{
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, AccountId),
		AccountDetails(AccountId, u64, Vec<u8>, Balance),
		AccBalance(AccountId, Balance),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		OriginNotSigned,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.


		/// An example dispatchable that may throw a custom error
		#[weight = 10_000]
		pub fn create_task(origin, task_duration: u64, task_des: Vec<u8>, task_cost: BalanceOf<T>) -> dispatch::DispatchResult {
		 let sender = ensure_signed(origin)?;
		 let temp= TaskDetails {
			  client:sender.clone(),
			  worker_id:None,
			  dur:task_duration.clone(),
			  des:task_des.clone(),
			  cost:task_cost.clone(),
		  };
		  TaskStorage::<T>::insert(task_des.clone(), temp);
		  Self::deposit_event(RawEvent::AccountDetails(sender, task_duration.clone(), task_des.clone(), task_cost.clone()));
		  Ok(())
		}

		#[weight = 10_000]
		pub fn get_account_balance(origin) -> dispatch::DispatchResult {

			// To check balance of an account
			// 1. Returns the account balance
			// 2. Store the balances in a map
			// 3. if the balance of the accountId already exists in the map, then get that value and return it
			// 4. else make a call using the Currency::total_balance function to get the account balance and
			//  store it in the map and also return the value

			let sender = ensure_signed(origin)?;

			let current_balance = T::Currency::total_balance(&sender);
			AccountBalances::<T>::insert(&sender, &current_balance);

			debug::info!("Account Balance: {:?}", current_balance);
			Self::deposit_event(RawEvent::AccBalance(sender, current_balance));

			// let result = AccountBalances::<T>::get(&sender);
			// debug::info!("Account Balance: {:?}", result);
			Ok(())
		}

		#[weight = 10_000]
		pub fn get_data_from_store(origin, task_des: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			let acc_balance = AccountBalances::<T>::get(&sender);
			debug::info!("get_data_from_store balance: {:?}", acc_balance);

			let task_details = TaskStorage::<T>::get(&task_des);
			debug::info!("get_data_from_store taskstore: {:?}", task_details.dur);

			Ok(())
		}
	}
}
