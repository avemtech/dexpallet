#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use sp_runtime::RuntimeDebug;
use codec::{Encode, Decode, MaxEncodedLen};
use scale_info::TypeInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[derive(Clone, Copy, PartialEq, Eq, Encode, Decode, Default, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct ChainId([u8; 8]);

#[derive(Clone, Copy, PartialEq, Eq, Encode, Decode, Default, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct AssetId([u8; 32]);

#[derive(Clone, Copy, PartialEq, Eq, Encode, Decode, Default, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct ForeignAccount([u8; 32]);

#[derive(Clone, Copy, PartialEq, Eq, Encode, Decode, Default, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct PriceValue {
	pub price: u128,
	pub value: u128,
}

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
    #[pallet::getter(fn account_chain_id_account)]
    pub(super) type AccountForeignAccount<T: Config> = StorageDoubleMap<_,
		Blake2_128Concat, T::AccountId,
		Blake2_128Concat, ChainId,
		ForeignAccount, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn account_pair_order)]
	pub(super) type Orderbook<T: Config> = StorageNMap<
	    _,
	    (
	        NMapKey<Blake2_128Concat, T::AccountId>,	// seller
	        NMapKey<Blake2_128Concat, AssetId>,			// sell assetId
	        NMapKey<Blake2_128Concat, AssetId>,			// buy assetId
	    ),
	    PriceValue,
	    ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SetForeignAccount(T::AccountId, ChainId, ForeignAccount),
		SetOrder(AssetId, AssetId, u128, u128),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(50_000_000)]
		pub fn set_foreign_account(origin: OriginFor<T>, chain_id: ChainId, foreign_account: ForeignAccount) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            <AccountForeignAccount<T>>::insert(&sender, chain_id, foreign_account);
            Self::deposit_event(Event::SetForeignAccount(sender, chain_id, foreign_account));
			Ok(().into())
		}

		#[pallet::weight(50_000_000)]
		pub fn set_order(origin: OriginFor<T>, sell_asset_id: AssetId, buy_asset_id: AssetId, price: u128, value: u128) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
			let price_value = PriceValue{
				price: price,
				value: value,
			};

            <Orderbook<T>>::insert((sender, sell_asset_id, buy_asset_id), price_value);
            Self::deposit_event(Event::SetOrder(sell_asset_id, buy_asset_id, price, value));
			Ok(().into())
		}

		#[pallet::weight(50_000_000)]
		pub fn remove_order(origin: OriginFor<T>, sell_asset_id: AssetId, buy_asset_id: AssetId) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			<Orderbook<T>>::remove((sender, sell_asset_id, buy_asset_id));
			Ok(().into())
		}

		#[pallet::weight(50_000_000)]
		pub fn remove_orders_for_sell_asset(origin: OriginFor<T>, sell_asset_id: AssetId) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			<Orderbook<T>>::clear_prefix((sender, sell_asset_id), u32::max_value(), None);
			Ok(().into())
		}

		#[pallet::weight(50_000_000)]
		pub fn remove_orders(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			<Orderbook<T>>::clear_prefix((sender,), u32::max_value(), None);
			Ok(().into())
		}
	}
}
