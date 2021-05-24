#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
use frame_support::{
	debug, decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
	traits::{Currency, ExistenceRequirement, ReservableCurrency},
};
use frame_system::ensure_signed;
use pallet_balances;
use pallet_staking;
use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(Encode, Decode, Default)]
pub struct TaskDetails<AccountId, Balance> {
	task_id: u128,
	client: AccountId,
	worker_id: Option<AccountId>,
	dur: u64,
	des: Vec<u8>,
	cost: Balance,
}

#[derive(Encode, Decode, Default, Debug, PartialEq, Clone, Eq)]
pub struct TransferDetails<AccountId, Balance> {
	transfer_from: AccountId,
	from_before: Balance,
	from_after: Balance,
	transfer_to: AccountId,
	to_before: Balance,
	to_after: Balance,
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
			map hasher(blake2_128_concat) u128 => TaskDetails<T::AccountId, BalanceOf<T>>;
			AccountBalances get(fn get_account_balances):
			map hasher(blake2_128_concat) T::AccountId => BalanceOf<T>;
			Count get(fn get_count): u128 = 0;
			Transfers get(fn get_transfers): Vec<TransferDetails<T::AccountId, BalanceOf<T>>>;
			StakerStorage get(fn staker_list):
			map hasher(blake2_128_concat) u128 => Vec<T::AccountId>;
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
		TaskCreated(AccountId, u128, u64, Vec<u8>, Balance),
		AccBalance(AccountId, Balance),
		CountIncreased(u128),
		TransferMoney(AccountId, Balance, Balance, AccountId, Balance, Balance),
		StakerAdded(AccountId),
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
		NotEnoughBalance,
		TaskDoesNotExist,
		AlreadyMember,
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
		 let current_count = Self::get_count();
		 let temp= TaskDetails {
			  task_id: current_count.clone(),
			  client:sender.clone(),
			  worker_id:None,
			  dur:task_duration.clone(),
			  des:task_des.clone(),
			  cost:task_cost.clone(),
		  };
		  TaskStorage::<T>::insert(current_count.clone(), temp);
		  Self::deposit_event(RawEvent::TaskCreated(sender, current_count.clone(), task_duration.clone(), task_des.clone(), task_cost.clone()));
		  Count::put(current_count + 1);
		  Ok(())
		}

		#[weight = 10_000]
		pub fn add_staker(origin, task_id: u128) -> dispatch::DispatchResult {
			let staker = ensure_signed(origin)?;
			ensure!(TaskStorage::<T>::contains_key(&task_id), Error::<T>::TaskDoesNotExist);
			
			let mut temp_staker_list = Self::staker_list(&task_id);
			debug::info!("Calling function using get method {:?}", &temp_staker_list);

			match temp_staker_list.binary_search(&staker) {
				// If the search succeeds, the caller is already a member, so just return
				Ok(_) => Err(Error::<T>::AlreadyMember.into()),
				// If the search fails, the caller is not a member and we learned the index where
				// they should be inserted
				Err(index) => {
					temp_staker_list.insert(index, staker.clone());
					StakerStorage::<T>::insert(task_id.clone(), temp_staker_list);
					Self::deposit_event(RawEvent::StakerAdded(staker.clone()));
					Ok(())
				}
			}
		}

		#[weight = 10_000]
		pub fn get_account_balance(origin) -> dispatch::DispatchResult {

			// To check balance of an account
			// 1. Returns the account balance
			// 2. Store the balances in a map
			// 3. if the balance of the accountId already exists in the map, then get that value and return it
			// 4. else make a call using the Currency::total_balance function to get the account balance and
			//  store it in the map and also return the value

			let result;
			let current_balance;
			let sender = ensure_signed(origin)?;

			result = AccountBalances::<T>::contains_key(&sender);
			if !result {
				current_balance = T::Currency::total_balance(&sender);
				AccountBalances::<T>::insert(&sender, &current_balance);
			} else {
				current_balance = AccountBalances::<T>::get(&sender);
			}

			debug::info!("Account Balance: {:?}", current_balance);
			Self::deposit_event(RawEvent::AccBalance(sender, current_balance));
			Ok(())
		}

		#[weight = 10_000]
		pub fn get_data_from_store(origin, task_id: u128) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			let acc_balance = AccountBalances::<T>::get(&sender);
			debug::info!("get_data_from_store balance: {:?}", acc_balance);

			let task_details = TaskStorage::<T>::get(&task_id);
			debug::info!("get_data_from_store taskstore: {:?}", task_details.dur);

			Ok(())
		}

		#[weight = 10_000]
		pub fn increase_counter(origin) {
			ensure_signed(origin)?;
			let current_count = Self::get_count();
			Count::put(current_count + 1);
			Self::deposit_event(RawEvent::CountIncreased(Self::get_count()));
		}



		#[weight = 10_000]
		pub fn transfer_money(origin, to: T::AccountId, transfer_amount: BalanceOf<T>) -> dispatch::DispatchResult {
			// 1. Transfer Money
			// 2. Check if the sender has enough funds to send money else throw Error
			// 2. Store the details in a struct
			// 3. Store the details in a vec
			let sender = ensure_signed(origin)?;
			let sender_account_balance = T::Currency::total_balance(&sender);

			// let is_valid_to_transfer = sender_account_balance.clone() < transfer_amount.clone();
			// debug::info!("is_valid_to_transfer {:?}", is_valid_to_transfer);
			// ensure!(!is_valid_to_transfer, Error::<T>::NotEnoughBalance);

			let to_account_balance = T::Currency::total_balance(&to);

			let result = T::Currency::transfer(&sender, &to, transfer_amount, ExistenceRequirement::KeepAlive)?;
			debug::info!("Transfer Result {:?}", result);

			let updated_sender_account_balance = T::Currency::total_balance(&sender);
			let updated_to_account_balance = T::Currency::total_balance(&to);
			Self::deposit_event(RawEvent::CountIncreased(Self::get_count()));

			// Initializing a vec and storing the details is a Vec
			let mut details: Vec<TransferDetails<T::AccountId, BalanceOf<T>>> = Vec::new();
			let transfer_details = TransferDetails {
				transfer_from: sender.clone(),
				from_before: sender_account_balance.clone(),
				from_after: updated_sender_account_balance.clone(),
				transfer_to: to.clone(),
				to_before: to_account_balance.clone(),
				to_after: updated_to_account_balance.clone(),
			};
			details.push(transfer_details);
			Transfers::<T>::put(details);
			debug::info!("Transfer Details Sender: {:#?}", &sender);
			debug::info!("Transfer Details Before Balance{:#?}", sender_account_balance.clone());
			debug::info!("Transfer Details After Balance: {:#?}", updated_sender_account_balance.clone());
			debug::info!("Transfer Details To Account: {:#?}", &to);
			debug::info!("Transfer Details Before Balance {:#?}", to_account_balance.clone());
			debug::info!("Transfer Details After Balance: {:#?}", updated_to_account_balance.clone());
			let transfers_in_store = Self::get_transfers();
			debug::info!("Transfer Details From Vec: {:#?}", &transfers_in_store[0]);
			Self::deposit_event(RawEvent::TransferMoney(sender.clone(), sender_account_balance.clone(), updated_sender_account_balance.clone(), to.clone(), to_account_balance.clone(), updated_to_account_balance.clone()));
			Ok(())
		}

	}
}
