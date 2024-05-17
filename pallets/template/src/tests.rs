use crate::{mock::*, Bid, Event};
use frame_support::pallet_prelude::Weight;
use frame_support::{assert_ok, traits::Hooks};
use sp_runtime::AccountId32;

#[test]
fn new_bid_should_work() {
    new_test_ext().execute_with(|| {

        System::set_block_number(2);

        let seller = RuntimeOrigin::signed(AccountId::from(AccountId32::from(
            b"000000000000000000000ALICE000000".clone(),
        )));
        let energy_quantity = 2; // in KW/Hour
        let starting_price = 1_000;
        let auction_period = 5; // in minutes

        let execution_block = System::block_number() + 50;

        assert_ok!(EnergyBiddingModule::new(
            seller,
            energy_quantity,
            starting_price,
            auction_period
        ));

        let auction = EnergyBiddingModule::auctions(0).expect("return indexed auction");

        assert_eq!(
            auction.seller_id,
            AccountId::from(AccountId32::from(
                b"000000000000000000000ALICE000000".clone(),
            ))
        );
        assert_eq!(auction.quantity, energy_quantity);
        assert_eq!(auction.starting_bid.bid, starting_price);

        let seller_auction_info = EnergyBiddingModule::auctions_of(AccountId::from(
            AccountId32::from(b"000000000000000000000ALICE000000".clone()),
        ))
        .expect("return seller's auction information");

        assert_eq!(
            seller_auction_info.participant_id.unwrap(),
            AccountId32::from(b"000000000000000000000ALICE000000".clone())
        );

        assert!(
            EnergyBiddingModule::auction_execution_queue(execution_block, auction.auction_id)
                .is_some()
        );

        System::assert_has_event(RuntimeEvent::EnergyBiddingModule(Event::AuctionCreated {
            auction_id: auction.auction_id,
            seller_id: auction.seller_id,
            energy_quantity: auction.quantity,
            starting_price,
        }));
    })
}

#[test]
fn cancel_should_work() {
    new_test_ext().execute_with(|| {

        System::set_block_number(2);

        let seller = RuntimeOrigin::signed(AccountId::from(AccountId32::from(
            b"000000000000000000000ALICE000000".clone(),
        )));
        let energy_quantity = 2; // in KWH
        let starting_price = 1_000;
        let auction_period = 5; // in minutes

        let execution_block = System::block_number() + 50;

        assert_ok!(EnergyBiddingModule::new(
            seller.clone(),
            energy_quantity,
            starting_price,
            auction_period
        ));

        let auction = EnergyBiddingModule::auctions(0).expect("return indexed auction");

        assert_ok!(EnergyBiddingModule::cancel(
            seller.clone(),
            auction.auction_id
        ));

        assert!(EnergyBiddingModule::auctions(auction.auction_id).is_none());

        assert!(
            EnergyBiddingModule::auctions_of(AccountId::from(AccountId32::from(
                b"000000000000000000000ALICE000000".clone(),
            )))
            .unwrap()
            .auctions
            .get(auction.auction_id as usize)
            .is_none()
        );

        assert!(
            EnergyBiddingModule::auction_execution_queue(execution_block, auction.auction_id)
                .is_none()
        );

        System::assert_has_event(RuntimeEvent::EnergyBiddingModule(Event::AuctionCanceled {
            auction_id: auction.auction_id,
            seller_id: auction.seller_id,
            energy_quantity: auction.quantity,
            starting_price: auction.starting_bid.bid,
        }));
    });
}


#[test]
fn bidding_should_work() {
    new_test_ext().execute_with(|| {

        System::set_block_number(2);

        let seller = RuntimeOrigin::signed(AccountId::from(AccountId32::from(
            b"000000000000000000000ALICE000000".clone(),
        )));
        let energy_quantity = 2; // in KW/Hour
        let starting_price = 1_000;
        let auction_period = 5; // in minutes

        assert_ok!(EnergyBiddingModule::new(
            seller.clone(),
            energy_quantity,
            starting_price,
            auction_period
        ));

        let mut auction = EnergyBiddingModule::auctions(0).expect("return indexed auction");

        let buyer = RuntimeOrigin::signed(AccountId::from(AccountId32::from(
            b"000000000000000000000BOB00000000".clone(),
        )));
        let auction_id = auction.auction_id;
        let new_bid = 10_000;

        assert_ok!(EnergyBiddingModule::bid(buyer.clone(), auction_id, new_bid));

        auction = EnergyBiddingModule::auctions(auction_id).expect("return indexed auction");
        assert_eq!(auction.highest_bid.bid, new_bid);
        assert_eq!(
            auction.highest_bid.bidder,
            AccountId32::from(b"000000000000000000000BOB00000000".clone())
        );

        assert_eq!(
            EnergyBiddingModule::auctions_of(AccountId::from(AccountId32::from(
                b"000000000000000000000BOB00000000".clone(),
            )))
            .unwrap()
            .auctions
            .get(auction.auction_id as usize)
            .unwrap()
            .highest_bid
            .bidder,
            AccountId32::from(b"000000000000000000000BOB00000000".clone())
        );

        assert!(
            EnergyBiddingModule::auctions_of(AccountId::from(AccountId32::from(
                b"000000000000000000000ALICE000000".clone(),
            )))
            .unwrap()
            .auctions
            .get(auction.auction_id as usize)
            .is_some()
        );

        System::assert_has_event(RuntimeEvent::EnergyBiddingModule(Event::AuctionBidAdded {
            auction_id: auction.auction_id,
            seller_id: auction.seller_id,
            energy_quantity: auction.quantity,
            bid: Bid {
                bidder: AccountId32::from(b"000000000000000000000BOB00000000".clone()),
                bid: new_bid,
            },
        }));
    });
}

#[test]
fn on_bid_ended_should_work() {
    new_test_ext().execute_with(|| {

        System::set_block_number(2);

        let seller = RuntimeOrigin::signed(AccountId::from(AccountId32::from(
            b"000000000000000000000ALICE000000".clone(),
        )));
        let energy_quantity = 2;
        let starting_price = 1_000;
        let auction_period = 5;

        assert_ok!(EnergyBiddingModule::new(
            seller.clone(),
            energy_quantity,
            starting_price,
            auction_period
        ));

        let mut auction = EnergyBiddingModule::auctions(0).expect("return indexed auction");

        let buyer = RuntimeOrigin::signed(AccountId::from(AccountId32::from(
            b"000000000000000000000BOB00000000".clone(),
        )));
        let auction_id = auction.auction_id;
        let new_bid = 10_000;

        assert_ok!(EnergyBiddingModule::bid(buyer.clone(), auction_id, new_bid));
        auction = EnergyBiddingModule::auctions(0).expect("return indexed auction");

        // fast forward block production to a block after auction execution block height
        let execution_block = System::block_number() + 50;
        System::set_block_number(52);

        assert_eq!(
            EnergyBiddingModule::on_initialize(execution_block),
            Weight::from_all(100_000_000u64)
        );
        EnergyBiddingModule::on_finalize(execution_block);

        assert!(
            EnergyBiddingModule::auction_execution_queue(execution_block, auction.auction_id)
                .is_none()
        );

        System::assert_has_event(RuntimeEvent::EnergyBiddingModule(Event::AuctionMatched {
            auction_id: auction.auction_id,
            seller_id: auction.seller_id.clone(),
            energy_quantity: auction.quantity,
            starting_price: auction.starting_bid.bid,
            highest_bid: auction.highest_bid.clone(),
            matched_at: System::block_number(),
        }));

        System::assert_has_event(RuntimeEvent::EnergyBiddingModule(Event::AuctionExecuted {
            auction_id: auction.auction_id,
            seller_id: auction.seller_id,
            buyer_id: auction.highest_bid.bidder,
            energy_quantity: auction.quantity,
            starting_price: auction.starting_bid.bid,
            highest_bid: auction.highest_bid.bid,
            executed_at: System::block_number(),
        }));
    });
}