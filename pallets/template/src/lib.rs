#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use crate::pallet::sp_runtime::{traits::AtLeast32BitUnsigned, FixedPointOperand};
    use frame_support::{
        dispatch::{fmt::Debug, Codec, EncodeLike},
        pallet_prelude::*,
        sp_runtime,
    };
    use frame_system::pallet_prelude::*;
    use scale_info::prelude::{vec, vec::Vec};

    const STORAGE_VERSION: frame_support::traits::StorageVersion =
        frame_support::traits::StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::without_storage_info]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);
    #[pallet::config]
    pub trait Config<I: 'static = ()>: frame_system::Config {
        type RuntimeEvent: From<Event<Self, I>>
            + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type AuctionId: Parameter
            + Member
            + AtLeast32BitUnsigned
            + Codec
            + Default
            + Copy
            + MaybeSerializeDeserialize
            + Debug
            + MaxEncodedLen
            + EncodeLike<u64>
            + TypeInfo
            + FixedPointOperand
            + From<u64>;

        type Quantity: Parameter
            + Member
            + AtLeast32BitUnsigned
            + Codec
            + Default
            + Copy
            + MaybeSerializeDeserialize
            + Debug
            + MaxEncodedLen
            + EncodeLike<u128>
            + TypeInfo
            + FixedPointOperand
            + From<u128>;

        type Price: Parameter
            + Member
            + AtLeast32BitUnsigned
            + Codec
            + Default
            + Copy
            + MaybeSerializeDeserialize
            + Debug
            + MaxEncodedLen
            + EncodeLike<u128>
            + TypeInfo
            + FixedPointOperand
            + From<u128>;
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
    pub struct Bid<AccountId, Price> {
        pub bidder: AccountId,
        pub bid: Price,
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
    pub enum AuctionStatus {
        Open,
        Closed,
    }
    impl Default for AuctionStatus {
        fn default() -> Self {
            AuctionStatus::Open
        }
    }

    #[derive(Clone, Encode, Decode, Default, Eq, PartialEq, RuntimeDebug, TypeInfo)]
    pub struct AuctionData<AccountId, AuctionId, Bid, BlockNumber, Quantity, Tier> {
        pub auction_id: AuctionId,
        pub seller_id: AccountId,
        pub quantity: Quantity,
        pub starting_bid: Bid,
        pub bids: Vec<Bid>,
        pub auction_period: BlockNumber,
        pub auction_status: AuctionStatus,
        pub start_at: BlockNumber,
        pub end_at: BlockNumber,
        pub highest_bid: Bid,
        pub auction_category: Tier,
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
    pub struct Tier {
        pub level: u32,
    }
    impl Default for Tier {
        fn default() -> Self {
            Tier { level: 1 }
        }
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
    pub enum PartyType {
        Seller,
        Buyer,
    }
    impl Default for PartyType {
        fn default() -> Self {
            PartyType::Seller
        }
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
    pub struct AuctionInfo<AccountId, AuctionId, Bid, BlockNumber, Tier, PartyType, Quantity> {
        pub participant_id: Option<AccountId>,
        pub party_type: PartyType,
        pub auctions: Vec<AuctionData<AccountId, AuctionId, Bid, BlockNumber, Quantity, Tier>>, /* Maximum* length of 5 */
    }

    #[pallet::storage]
    #[pallet::getter(fn auctions_index)]
    pub(super) type AuctionIndex<T: Config<I>, I: 'static = ()> = StorageValue<_, T::AuctionId>;

    #[pallet::storage]
    #[pallet::getter(fn auctions_of)]
    pub(super) type AuctionsOf<T: Config<I>, I: 'static = ()> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        AuctionInfo<
            T::AccountId,
            T::AuctionId,
            Bid<T::AccountId, T::Price>,
            BlockNumberFor<T>,
            Tier,
            PartyType,
            T::Quantity,
        >,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn auctions)]
    pub(super) type Auctions<T: Config<I>, I: 'static = ()> = StorageMap<
        _,
        Twox64Concat,
        T::AuctionId,
        AuctionData<
            T::AccountId,
            T::AuctionId,
            Bid<T::AccountId, T::Price>,
            BlockNumberFor<T>,
            T::Quantity,
            Tier,
        >,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn auction_execution_queue)]
    pub(super) type AuctionsExecutionQueue<T: Config<I>, I: 'static = ()> = StorageDoubleMap<
        _,
        Twox64Concat,
        BlockNumberFor<T>,
        Blake2_128Concat,
        T::AuctionId,
        (),
        OptionQuery,
    >;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config<I>, I: 'static = ()> {
        pub auction_index: T::AuctionId,
    }

    impl<T: Config<I>, I: 'static> Default for GenesisConfig<T, I> {
        fn default() -> Self {
            Self {
                auction_index: Default::default(),
            }
        }
    }

	use frame_support::traits::BuildGenesisConfig;
    #[pallet::genesis_build]
    impl<T: Config<I>, I: 'static> BuildGenesisConfig for GenesisConfig<T, I> {
        fn build(&self) {
            let initial_id = self.auction_index;
            <AuctionIndex<T, I>>::put(initial_id);
        }
    }

    #[pallet::hooks]
    impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Pallet<T, I> {
        fn on_initialize(_now: BlockNumberFor<T>) -> Weight {
            Weight::from_all(100_000_000u64)
        }

        fn on_finalize(now: BlockNumberFor<T>) {
            for (auction_id, _) in AuctionsExecutionQueue::<T, I>::drain_prefix(now) {
                if let Some(auction) = Auctions::<T, I>::get(auction_id) {
                    Self::on_auction_ended(auction.auction_id);
                }
            }
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config<I>, I: 'static = ()> {
        AuctionCreated {
            auction_id: T::AuctionId,
            seller_id: T::AccountId,
            energy_quantity: T::Quantity,
            starting_price: T::Price,
        },

        AuctionBidAdded {
            auction_id: T::AuctionId,
            seller_id: T::AccountId,
            energy_quantity: T::Quantity,
            bid: Bid<T::AccountId, T::Price>,
        },

        AuctionMatched {
            auction_id: T::AuctionId,
            seller_id: T::AccountId,
            energy_quantity: T::Quantity,
            starting_price: T::Price,
            highest_bid: Bid<T::AccountId, T::Price>,
            matched_at: BlockNumberFor<T>,
        },

        AuctionExecuted {
            auction_id: T::AuctionId,
            seller_id: T::AccountId,
            buyer_id: T::AccountId,
            energy_quantity: T::Quantity,
            starting_price: T::Price,
            highest_bid: T::Price,
            executed_at: BlockNumberFor<T>,
        },

        AuctionCanceled {
            auction_id: T::AuctionId,
            seller_id: T::AccountId,
            energy_quantity: T::Quantity,
            starting_price: T::Price,
        },
    }

    //////////////////////
    // Pallet errors   //
    /////////////////////
    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T, I = ()> {
        AuctionDoesNotExist,

        AuctionIsOver,

        InsuffficientAttachedDeposit,
    }

    #[pallet::call]
    impl<T: Config<I>, I: 'static> Pallet<T, I> {
        #[pallet::call_index(0)]
        #[pallet::weight(100_000_000)]
        pub fn new(
            origin: OriginFor<T>,
            energy_quantity: u128, // in KWH
            starting_price: u128,  // in parachain native token
            auction_period: u16,   // in minutes
        ) -> DispatchResult {
            let seller = ensure_signed(origin)?;

            let current_auction_id = AuctionIndex::<T, I>::get().unwrap_or_default();

            let auction_period_in_block_number = (auction_period.checked_mul(60).unwrap())
                .checked_div(6)
                .unwrap()
                .into();

            let starting_block_number = <frame_system::Pallet<T>>::block_number();

            let ending_block_number = starting_block_number + auction_period_in_block_number;

            let starting_bid = Bid::<T::AccountId, T::Price> {
                bidder: seller.clone(),
                bid: T::Price::from(starting_price),
            };

            let category;
            if energy_quantity < 5 {
                category = Tier::default()
            } else {
                category = Tier { level: 2 }
            }

            let auction_data = AuctionData {
                auction_id: current_auction_id,
                seller_id: seller.clone(),
                quantity: T::Quantity::from(energy_quantity),
                starting_bid: starting_bid.clone(),
                bids: vec![starting_bid.clone()],
                auction_period: auction_period_in_block_number,
                auction_status: AuctionStatus::default(),
                start_at: starting_block_number,
                end_at: ending_block_number,
                highest_bid: starting_bid,
                auction_category: category,
            };

            let mut seller_auction_info =
                AuctionsOf::<T, I>::get(seller.clone()).unwrap_or(AuctionInfo {
                    participant_id: None,
                    party_type: PartyType::Seller,
                    auctions: vec![],
                });

            if seller_auction_info.auctions.len() > 5 {
                seller_auction_info.auctions.pop();
            }

            seller_auction_info.auctions.push(auction_data.clone());

            seller_auction_info = AuctionInfo {
                participant_id: Some(seller.clone()),
                party_type: PartyType::Seller,
                auctions: seller_auction_info.auctions,
            };
            AuctionsOf::<T, I>::insert(&seller, seller_auction_info);

            AuctionsExecutionQueue::<T, I>::insert(
                auction_data.end_at,
                auction_data.auction_id,
                (),
            );

            Auctions::<T, I>::insert(&auction_data.auction_id, auction_data.clone());

            let next_id = current_auction_id + T::AuctionId::from(1u64);
            AuctionIndex::<T, I>::set(Some(next_id));

            Self::deposit_event(Event::AuctionCreated {
                auction_id: auction_data.auction_id,
                seller_id: seller,
                energy_quantity: auction_data.quantity,
                starting_price: auction_data.starting_bid.bid,
            });

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(100_000_000)]
        pub fn cancel(origin: OriginFor<T>, auction_id: T::AuctionId) -> DispatchResult {

            let _signer = ensure_signed(origin)?;

            ensure!(
                Auctions::<T, I>::contains_key(auction_id),
                Error::<T, I>::AuctionDoesNotExist
            );

            let mut auction_data = Auctions::<T, I>::get(auction_id).expect("data of auction");

            ensure!(
                matches!(auction_data.auction_status, AuctionStatus::Open),
                Error::<T, I>::AuctionIsOver
            );

            auction_data.auction_status = AuctionStatus::Closed;

            Auctions::<T, I>::remove(auction_data.auction_id);

            let mut sellers_auction_info =
                AuctionsOf::<T, I>::get(auction_data.seller_id.clone()).expect("info of seller");

            let index = sellers_auction_info
                .auctions
                .iter()
                .position(|x| x.auction_id.clone() == auction_id)
                .expect("index of auction");

            sellers_auction_info.auctions.remove(index);
            AuctionsOf::<T, I>::insert(auction_data.seller_id.clone(), sellers_auction_info);

            AuctionsExecutionQueue::<T, I>::remove(auction_data.end_at, auction_data.auction_id);

            Self::deposit_event(Event::AuctionCanceled {
                auction_id: auction_data.auction_id,
                seller_id: auction_data.seller_id,
                energy_quantity: auction_data.quantity,
                starting_price: auction_data.starting_bid.bid,
            });

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(100_000_000)]
        pub fn bid(origin: OriginFor<T>, auction_id: T::AuctionId, bid: u128) -> DispatchResult {
            let buyer_id = ensure_signed(origin)?;

            ensure!(
                Auctions::<T, I>::contains_key(auction_id),
                Error::<T, I>::AuctionDoesNotExist
            );

            let mut auction_data = Auctions::<T, I>::get(auction_id).expect("data of auction");

            ensure!(
                matches!(auction_data.auction_status, AuctionStatus::Open),
                Error::<T, I>::AuctionIsOver
            );

            let new_bid = Bid::<T::AccountId, T::Price> {
                bidder: buyer_id.clone(),
                bid: bid.into(),
            };

            if new_bid.bid > auction_data.bids[0].bid {

                auction_data.bids.insert(0, new_bid.clone());
                auction_data.highest_bid = new_bid.clone();
                Auctions::<T, I>::insert(auction_data.auction_id, auction_data.clone());
            }

            let buyer_auction_info = AuctionsOf::<T, I>::get(buyer_id.clone());

            match buyer_auction_info {

                Some(mut auction_info) => {
                    for (index, auction) in auction_info.auctions.clone().into_iter().enumerate() {

                        if auction_info.auctions.len() >= 5 {
                            auction_info.auctions.pop();
                        }

                        if auction.auction_id == auction_id {

                            auction_info.auctions.insert(index, auction_data.clone());

                            AuctionsOf::<T, I>::insert(
                                &buyer_id,
                                AuctionInfo {
                                    participant_id: Some(buyer_id.clone()),
                                    party_type: PartyType::Seller,
                                    auctions: auction_info.auctions.clone(),
                                },
                            )
                        }
                    }
                }

                None => {

                    let mut auction_info =
                        AuctionsOf::<T, I>::get(buyer_id.clone()).unwrap_or(AuctionInfo {
                            participant_id: None,
                            party_type: PartyType::Seller,
                            auctions: vec![],
                        });

                    auction_info.auctions.push(auction_data.clone());

                    AuctionsOf::<T, I>::insert(
                        &buyer_id,
                        AuctionInfo {
                            participant_id: Some(buyer_id.clone()),
                            party_type: PartyType::Seller,
                            auctions: auction_info.auctions.clone(),
                        },
                    )
                }
            }

            let mut seller_auction_info =
                AuctionsOf::<T, I>::get(auction_data.clone().seller_id).expect("info of seller");

            for (index, auction) in seller_auction_info.auctions.clone().into_iter().enumerate() {

                if seller_auction_info.auctions.len() >= 5 {
                    seller_auction_info.auctions.pop();
                }

                if auction.auction_id == auction_id {

                    seller_auction_info
                        .auctions
                        .insert(index, auction_data.clone());

                    AuctionsOf::<T, I>::insert(
                        &buyer_id,
                        AuctionInfo {
                            participant_id: Some(buyer_id.clone()),
                            party_type: PartyType::Seller,
                            auctions: seller_auction_info.auctions.clone(),
                        },
                    )
                }
            }

            Auctions::<T, I>::insert(&auction_data.auction_id, auction_data.clone());

            Self::deposit_event(Event::AuctionBidAdded {
                auction_id: auction_data.auction_id,
                seller_id: auction_data.seller_id,
                energy_quantity: auction_data.quantity,
                bid: new_bid,
            });

            Ok(())
        }
    }

    impl<T: Config<I>, I: 'static> Pallet<T, I> {
        fn on_auction_ended(auction_id: T::AuctionId) {

            let auction_data = Auctions::<T, I>::take(auction_id).unwrap();
            let now = <frame_system::Pallet<T>>::block_number();

            Self::deposit_event(Event::AuctionMatched {
                auction_id: auction_data.auction_id,
                seller_id: auction_data.seller_id.clone(),
                energy_quantity: auction_data.quantity,
                starting_price: auction_data.starting_bid.bid,
                highest_bid: auction_data.highest_bid.clone(),
                matched_at: now,
            });

            Self::deposit_event(Event::AuctionExecuted {
                auction_id: auction_data.auction_id,
                seller_id: auction_data.seller_id,
                buyer_id: auction_data.highest_bid.bidder,
                energy_quantity: auction_data.quantity,
                starting_price: auction_data.starting_bid.bid,
                highest_bid: auction_data.highest_bid.bid,
                executed_at: now,
            });
        }
    }
}
