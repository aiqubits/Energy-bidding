//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{
	benchmarks, 
	whitelisted_caller,
	v2::*
};
use frame_system::RawOrigin;

benchmarks! {
	new {  //1、准备条件
	  let energy_quantity = 10;
	  let starting_price = 10;
	  let auction_period = 2;
	  let caller: T::AccountId = whitelisted_caller();
	//   let seller = RawOrigin::Signed(caller).into();

	//   RawOrigin::Signed(caller).into()
	}:{     //2、调用调度函数
	  let _ = Template::<T>::new(RawOrigin::Signed(caller).into(), energy_quantity, starting_price, auction_period);
	//   let _ = UseBenchmarkingDemo::<T>::new(RawOrigin::Signed(caller).into(), s.into(), Default::default());
	}
	verify {//3、进行验证
		// todo!   AccountId
		assert_eq!(<AuctionsOf<T>>::get::<<T as pallet::Config>::AccountId>(RawOrigin::Signed(caller).into(), energy_quantity, starting_price, auction_period), Default::default());

		// let auction = Template::<T>::auctions(0u128).expect("return indexed auction");
 		// assert_eq!(auction.seller_id, RawOrigin::Signed(caller).into());
 		// assert_eq!(auction.quantity, energy_quantity);
		// assert_eq!(auction.starting_bid.bid, starting_price);
		// assert_eq!(auction.auction_period, auction_period);
	}
		  
	// 使用mock中的new_test_ext
	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
  }

// #[benchmarks]
// mod benchmarks {
// 	use super::*;

// 	#[benchmark]
// 	fn new() {
// 		let energy_quantity = 5u128.into();
// 		let starting_price = 10u128.into();
// 		let auction_period = 2u16.into();
// 		let caller: T::AccountId = whitelisted_caller();

// 		let seller = RawOrigin::Signed(caller);

// 		Template::new(seller.clone(), energy_quantity, starting_price, auction_period);

// 		let auction = Template::auctions(0).expect("return indexed auction");

// 		assert_eq!(auction.seller_id, seller);
// 		assert_eq!(auction.quantity, energy_quantity);
//         assert_eq!(auction.starting_bid.bid, starting_price);
//         assert_eq!(auction.auction_period, auction_period);
// 	}

// 	#[benchmark]
// 	fn cancel() {
// 		let energy_quantity = 5u128.into();
// 		let starting_price = 10u128.into();
// 		let auction_period = 10u16.into();
// 		let caller: T::AccountId = whitelisted_caller();

// 		let seller = RawOrigin::Signed(caller);

// 		assert_ok!(Template::new(seller.clone(), energy_quantity, starting_price, auction_period));

// 		let auction = Template::auctions(0).expect("return indexed auction");

// 		Template::cancel(seller.clone(), auction.auction_id);

// 		let buyer = RuntimeOrigin::signed(AccountId::from(AccountId32::from(
//             b"000000000000000000000BOB00000000".clone(),
//         )));
//         let auction_id = auction.auction_id;
//         let new_bid = 1_000u128.into();

// 		assert_ok!(Template::bid(buyer.clone(), auction_id, new_bid));

// 		auction = Template::auctions(auction_id).expect("return indexed auction");

//         assert_eq!(auction.highest_bid.bid, new_bid);
//         assert_eq!(
//             auction.highest_bid.bidder,
//             AccountId32::from(b"000000000000000000000BOB00000000".clone())
//         );
		
// 	}

// 	#[benchmark]
// 	fn bid() {
// 		let energy_quantity = 5u32.into();
// 		let starting_price = 10u32.into();
// 		let auction_period = 1u32.into();
// 		let caller: T::AccountId = whitelisted_caller();

// 		let seller = RawOrigin::Signed(caller);

// 		assert_ok!(Template::new(seller.clone(), energy_quantity, starting_price, auction_period));

// 		let auction = Template::auctions(0).expect("return indexed auction");

// 		assert_eq!(Something::<T>::get(), Some(101u32));
// 	}

// 	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
// }
