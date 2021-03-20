//! Unit tests for the stp258_tokens module.

#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::{Event, *};

#[test]
fn base_unit_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(Stp258Tokens::base_unit(SETT), 10_000);
		assert_eq!(Stp258Tokens::base_unit(JUSD), 1_000);
	});
}

#[test]
fn minimum_balance_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(Stp258Tokens::minimum_balance(DNAR), 2);
		assert_eq!(Stp258Tokens::minimum_balance(SETT), 1 * 10_000);
		assert_eq!(Stp258Tokens::minimum_balance(JUSD), 1 * 1_000);
	});
}

#[test]
fn expand_supply_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob_n_serper_n_settpay()
		.build()
		.execute_with(|| {
			assert_ok!(Stp258Tokens::reserve(DNAR, &SERPER, 100));
			assert_ok!(Stp258Tokens::reserve(JUSD, &SERPER, 100 * 1_000));
			assert_eq!(Stp258Tokens::reserved_balance(DNAR, &ALICE), 100);
			assert_eq!(Stp258Tokens::reserved_balance(JUSD, &ALICE), 100 * 1_000);
			assert_eq!(Stp258Tokens::total_issuance(DNAR), 400);
			assert_eq!(Stp258Tokens::total_issuance(JUSD), 400 * 1_000);
			assert_ok!(Stp258Tokens::expand_supply(DNAR, JUSD, 40, 18_000)); 
			assert_eq!(Stp258Tokens::reserved_balance(JUSD, &SERPER), 140 * 1_000);
			assert_eq!(Stp258Tokens::reserved_balance(DNAR, &SERPER), 98);
			assert_eq!(Stp258Tokens::total_issuance(JUSD), 440 * 1_000);
		});
}

#[test]
fn contract_supply_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob_n_serper_n_settpay()
		.build()
		.execute_with(|| {
			assert_ok!(Stp258Tokens::reserve(DNAR, &SERPER, 100));
			assert_ok!(Stp258Tokens::reserve(JUSD, &SERPER, 100 * 1_000));
			assert_eq!(Stp258Tokens::reserved_balance(DNAR, &ALICE), 100);
			assert_eq!(Stp258Tokens::reserved_balance(JUSD, &ALICE), 100 * 1_000);
			assert_eq!(Stp258Tokens::total_issuance(DNAR), 400);
			assert_eq!(Stp258Tokens::total_issuance(JUSD), 400 * 1_000);
			assert_ok!(Stp258Tokens::contract_supply(DNAR, JUSD, 40, 11)); 
			assert_eq!(Stp258Tokens::reserved_balance(JUSD, &SERPER), 60 * 1_000);
			assert_eq!(Stp258Tokens::reserved_balance(DNAR, &SERPER), 104);
			assert_eq!(Stp258Tokens::total_issuance(JUSD), 360 * 1_000);
			assert_eq!(Stp258Tokens::total_issuance(DNAR), 404);
		});
}


#[test]
fn is_module_account_id_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(Stp258Tokens::is_module_account_id(&ALICE), false);
		assert_eq!(Stp258Tokens::is_module_account_id(&BOB), false);
		assert_eq!(Stp258Tokens::is_module_account_id(&TREASURY_ACCOUNT), false);
		assert_eq!(Stp258Tokens::is_module_account_id(&DustAccount::get()), true);
	});
}

#[test]
fn remove_dust_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(Stp258Tokens::deposit(DNAR, &ALICE, 100));
		assert_eq!(Stp258Tokens::total_issuance(DNAR), 100);
		assert_eq!(Accounts::<Runtime>::contains_key(ALICE, DNAR), true);
		assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 100);
		assert_eq!(System::providers(&ALICE), 1);
		assert_eq!(Accounts::<Runtime>::contains_key(DustAccount::get(), DNAR), false);
		assert_eq!(Stp258Tokens::free_balance(DNAR, &DustAccount::get()), 0);
		assert_eq!(System::providers(&DustAccount::get()), 0);

		// total is gte ED, will not handle dust
		assert_ok!(Stp258Tokens::withdraw(DNAR, &ALICE, 98));
		assert_eq!(Stp258Tokens::total_issuance(DNAR), 2);
		assert_eq!(Accounts::<Runtime>::contains_key(ALICE, DNAR), true);
		assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 2);
		assert_eq!(System::providers(&ALICE), 1);
		assert_eq!(Accounts::<Runtime>::contains_key(DustAccount::get(), DNAR), false);
		assert_eq!(Stp258Tokens::free_balance(DNAR, &DustAccount::get()), 0);
		assert_eq!(System::providers(&DustAccount::get()), 0);

		assert_ok!(Stp258Tokens::withdraw(DNAR, &ALICE, 1));

		// total is lte ED, will handle dust
		assert_eq!(Stp258Tokens::total_issuance(DNAR), 1);
		assert_eq!(Accounts::<Runtime>::contains_key(ALICE, DNAR), false);
		assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 0);
		assert_eq!(System::providers(&ALICE), 0);

		// will not handle dust for module account
		assert_eq!(Accounts::<Runtime>::contains_key(DustAccount::get(), DNAR), true);
		assert_eq!(Stp258Tokens::free_balance(DNAR, &DustAccount::get()), 1);
		assert_eq!(System::providers(&DustAccount::get()), 1);

		let dust_lost_event = Event::stp258_tokens(crate::Event::DustLost(ALICE, DNAR, 1));
		assert!(System::events().iter().any(|record| record.event == dust_lost_event));
	});
}

#[test]
fn set_lock_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob_n_serper_n_settpay()
		.build()
		.execute_with(|| {
			assert_ok!(Stp258Tokens::set_lock(ID_1, DNAR, &ALICE, 10));
			assert_eq!(Stp258Tokens::accounts(&ALICE, DNAR).frozen, 10);
			assert_eq!(Stp258Tokens::accounts(&ALICE, DNAR).frozen(), 10);
			assert_eq!(Stp258Tokens::locks(ALICE, DNAR).len(), 1);
			assert_ok!(Stp258Tokens::set_lock(ID_1, DNAR, &ALICE, 50));
			assert_eq!(Stp258Tokens::accounts(&ALICE, DNAR).frozen, 50);
			assert_eq!(Stp258Tokens::locks(ALICE, DNAR).len(), 1);
			assert_ok!(Stp258Tokens::set_lock(ID_2, DNAR, &ALICE, 60));
			assert_eq!(Stp258Tokens::accounts(&ALICE, DNAR).frozen, 60);
			assert_eq!(Stp258Tokens::locks(ALICE, DNAR).len(), 2);
		});
}

#[test]
fn extend_lock_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob_n_serper_n_settpay()
		.build()
		.execute_with(|| {
			assert_ok!(Stp258Tokens::set_lock(ID_1, DNAR, &ALICE, 10));
			assert_eq!(Stp258Tokens::locks(ALICE, DNAR).len(), 1);
			assert_eq!(Stp258Tokens::accounts(&ALICE, DNAR).frozen, 10);
			assert_ok!(Stp258Tokens::extend_lock(ID_1, DNAR, &ALICE, 20));
			assert_eq!(Stp258Tokens::locks(ALICE, DNAR).len(), 1);
			assert_eq!(Stp258Tokens::accounts(&ALICE, DNAR).frozen, 20);
			assert_ok!(Stp258Tokens::extend_lock(ID_2, DNAR, &ALICE, 10));
			assert_ok!(Stp258Tokens::extend_lock(ID_1, DNAR, &ALICE, 20));
			assert_eq!(Stp258Tokens::locks(ALICE, DNAR).len(), 2);
		});
}

#[test]
fn remove_lock_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob_n_serper_n_settpay()
		.build()
		.execute_with(|| {
			assert_ok!(Stp258Tokens::set_lock(ID_1, DNAR, &ALICE, 10));
			assert_ok!(Stp258Tokens::set_lock(ID_2, DNAR, &ALICE, 20));
			assert_eq!(Stp258Tokens::locks(ALICE, DNAR).len(), 2);
			assert_ok!(Stp258Tokens::remove_lock(ID_2, DNAR, &ALICE));
			assert_eq!(Stp258Tokens::locks(ALICE, DNAR).len(), 1);
		});
}

#[test]
fn frozen_can_limit_liquidity() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob_n_serper_n_settpay()
		.build()
		.execute_with(|| {
			assert_ok!(Stp258Tokens::set_lock(ID_1, DNAR, &ALICE, 90));
			assert_noop!(
				<Stp258Tokens as Stp258Currency<_>>::transfer(DNAR, &ALICE, &BOB, 11),
				Error::<Runtime>::LiquidityRestrictions,
			);
			assert_ok!(Stp258Tokens::set_lock(ID_1, DNAR, &ALICE, 10));
			assert_ok!(<Stp258Tokens as Stp258Currency<_>>::transfer(DNAR, &ALICE, &BOB, 11),);
		});
}

#[test]
fn can_reserve_is_correct() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob_n_serper_n_settpay()
		.build()
		.execute_with(|| {
			assert_eq!(Stp258Tokens::can_reserve(DNAR, &ALICE, 0), true);
			assert_eq!(Stp258Tokens::can_reserve(DNAR, &ALICE, 101), false);
			assert_eq!(Stp258Tokens::can_reserve(DNAR, &ALICE, 100), true);
		});
}

#[test]
fn reserve_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob_n_serper_n_settpay()
		.build()
		.execute_with(|| {
			assert_noop!(Stp258Tokens::reserve(DNAR, &ALICE, 101), Error::<Runtime>::BalanceTooLow,);
			assert_ok!(Stp258Tokens::reserve(DNAR, &ALICE, 0));
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 100);
			assert_eq!(Stp258Tokens::reserved_balance(DNAR, &ALICE), 0);
			assert_eq!(Stp258Tokens::total_balance(DNAR, &ALICE), 100);
			assert_ok!(Stp258Tokens::reserve(DNAR, &ALICE, 50));
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 50);
			assert_eq!(Stp258Tokens::reserved_balance(DNAR, &ALICE), 50);
			assert_eq!(Stp258Tokens::total_balance(DNAR, &ALICE), 100);
		});
}

#[test]
fn unreserve_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob_n_serper_n_settpay()
		.build()
		.execute_with(|| {
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 100);
			assert_eq!(Stp258Tokens::reserved_balance(DNAR, &ALICE), 0);
			assert_eq!(Stp258Tokens::unreserve(DNAR, &ALICE, 0), 0);
			assert_eq!(Stp258Tokens::unreserve(DNAR, &ALICE, 50), 50);
			assert_ok!(Stp258Tokens::reserve(DNAR, &ALICE, 30));
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 70);
			assert_eq!(Stp258Tokens::reserved_balance(DNAR, &ALICE), 30);
			assert_eq!(Stp258Tokens::unreserve(DNAR, &ALICE, 15), 0);
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 85);
			assert_eq!(Stp258Tokens::reserved_balance(DNAR, &ALICE), 15);
			assert_eq!(Stp258Tokens::unreserve(DNAR, &ALICE, 30), 15);
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 100);
			assert_eq!(Stp258Tokens::reserved_balance(DNAR, &ALICE), 0);
		});
}

#[test]
fn slash_reserved_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob_n_serper_n_settpay()
		.build()
		.execute_with(|| {
			assert_ok!(Stp258Tokens::reserve(DNAR, &ALICE, 50));
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 50);
			assert_eq!(Stp258Tokens::reserved_balance(DNAR, &ALICE), 50);
			assert_eq!(Stp258Tokens::total_issuance(DNAR), 400);
			assert_eq!(Stp258Tokens::slash_reserved(DNAR, &ALICE, 0), 0);
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 50);
			assert_eq!(Stp258Tokens::reserved_balance(DNAR, &ALICE), 50);
			assert_eq!(Stp258Tokens::total_issuance(DNAR), 400);
			assert_eq!(Stp258Tokens::slash_reserved(DNAR, &ALICE, 100), 50);
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 50);
			assert_eq!(Stp258Tokens::reserved_balance(DNAR, &ALICE), 0);
			assert_eq!(Stp258Tokens::total_issuance(DNAR), 350);
		});
}

#[test]
fn repatriate_reserved_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob_n_serper_n_settpay()
		.build()
		.execute_with(|| {
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 100);
			assert_eq!(Stp258Tokens::reserved_balance(DNAR, &ALICE), 0);
			assert_eq!(
				Stp258Tokens::repatriate_reserved(DNAR, &ALICE, &ALICE, 0, BalanceStatus::Free),
				Ok(0)
			);
			assert_eq!(
				Stp258Tokens::repatriate_reserved(DNAR, &ALICE, &ALICE, 50, BalanceStatus::Free),
				Ok(50)
			);
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 100);
			assert_eq!(Stp258Tokens::reserved_balance(DNAR, &ALICE), 0);

			assert_eq!(Stp258Tokens::free_balance(DNAR, &BOB), 100);
			assert_eq!(Stp258Tokens::reserved_balance(DNAR, &BOB), 0);
			assert_ok!(Stp258Tokens::reserve(DNAR, &BOB, 50));
			assert_eq!(Stp258Tokens::free_balance(DNAR, &BOB), 50);
			assert_eq!(Stp258Tokens::reserved_balance(DNAR, &BOB), 50);
			assert_eq!(
				Stp258Tokens::repatriate_reserved(DNAR, &BOB, &BOB, 60, BalanceStatus::Reserved),
				Ok(10)
			);
			assert_eq!(Stp258Tokens::free_balance(DNAR, &BOB), 50);
			assert_eq!(Stp258Tokens::reserved_balance(DNAR, &BOB), 50);

			assert_eq!(
				Stp258Tokens::repatriate_reserved(DNAR, &BOB, &ALICE, 30, BalanceStatus::Reserved),
				Ok(0)
			);
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 100);
			assert_eq!(Stp258Tokens::reserved_balance(DNAR, &ALICE), 30);
			assert_eq!(Stp258Tokens::free_balance(DNAR, &BOB), 50);
			assert_eq!(Stp258Tokens::reserved_balance(DNAR, &BOB), 20);

			assert_eq!(
				Stp258Tokens::repatriate_reserved(DNAR, &BOB, &ALICE, 30, BalanceStatus::Free),
				Ok(10)
			);
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 120);
			assert_eq!(Stp258Tokens::reserved_balance(DNAR, &ALICE), 30);
			assert_eq!(Stp258Tokens::free_balance(DNAR, &BOB), 50);
			assert_eq!(Stp258Tokens::reserved_balance(DNAR, &BOB), 0);
		});
}

#[test]
fn slash_draw_reserved_correct() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob_n_serper_n_settpay()
		.build()
		.execute_with(|| {
			assert_ok!(Stp258Tokens::reserve(DNAR, &ALICE, 50));
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 50);
			assert_eq!(Stp258Tokens::reserved_balance(DNAR, &ALICE), 50);
			assert_eq!(Stp258Tokens::total_issuance(DNAR), 400);

			assert_eq!(Stp258Tokens::slash(DNAR, &ALICE, 80), 0);
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 0);
			assert_eq!(Stp258Tokens::reserved_balance(DNAR, &ALICE), 20);
			assert_eq!(Stp258Tokens::total_issuance(DNAR), 320);

			assert_eq!(Stp258Tokens::slash(DNAR, &ALICE, 50), 30);
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 0);
			assert_eq!(Stp258Tokens::reserved_balance(DNAR, &ALICE), 0);
			assert_eq!(Stp258Tokens::total_issuance(DNAR), 300);
		});
}

#[test]
fn genesis_issuance_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob_n_serper_n_settpay()
		.build()
		.execute_with(|| {
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 100);
			assert_eq!(Stp258Tokens::free_balance(DNAR, &BOB), 100);
			assert_eq!(Stp258Tokens::total_issuance(DNAR), 400);
		});
}

#[test]
fn transfer_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob_n_serper_n_settpay()
		.build()
		.execute_with(|| {
			System::set_block_number(1);

			assert_ok!(Stp258Tokens::transfer(Some(ALICE).into(), BOB, DNAR, 50));
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 50);
			assert_eq!(Stp258Tokens::free_balance(DNAR, &BOB), 150);
			assert_eq!(Stp258Tokens::total_issuance(DNAR), 400);

			let transferred_event = Event::stp258_tokens(crate::Event::Transferred(DNAR, ALICE, BOB, 50));
			assert!(System::events().iter().any(|record| record.event == transferred_event));

			assert_noop!(
				Stp258Tokens::transfer(Some(ALICE).into(), BOB, DNAR, 60),
				Error::<Runtime>::BalanceTooLow,
			);
		});
}

#[test]
fn transfer_all_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob_n_serper_n_settpay()
		.build()
		.execute_with(|| {
			System::set_block_number(1);

			assert_ok!(Stp258Tokens::transfer_all(Some(ALICE).into(), BOB, DNAR));
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 0);
			assert_eq!(Stp258Tokens::free_balance(DNAR, &BOB), 200);

			let transferred_event = Event::stp258_tokens(crate::Event::Transferred(DNAR, ALICE, BOB, 100));
			assert!(System::events().iter().any(|record| record.event == transferred_event));
		});
}

#[test]
fn deposit_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob_n_serper_n_settpay()
		.build()
		.execute_with(|| {
			assert_ok!(Stp258Tokens::deposit(DNAR, &ALICE, 100));
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 200);
			assert_eq!(Stp258Tokens::total_issuance(DNAR), 500);

			assert_noop!(
				Stp258Tokens::deposit(DNAR, &ALICE, Balance::max_value()),
				Error::<Runtime>::TotalIssuanceOverflow,
			);
		});
}

#[test]
fn withdraw_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob_n_serper_n_settpay()
		.build()
		.execute_with(|| {
			assert_ok!(Stp258Tokens::withdraw(DNAR, &ALICE, 50));
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 50);
			assert_eq!(Stp258Tokens::total_issuance(DNAR), 350);

			assert_noop!(Stp258Tokens::withdraw(DNAR, &ALICE, 60), Error::<Runtime>::BalanceTooLow);
		});
}

#[test]
fn slash_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob_n_serper_n_settpay()
		.build()
		.execute_with(|| {
			// slashed_amount < amount
			assert_eq!(Stp258Tokens::slash(DNAR, &ALICE, 50), 0);
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 50);
			assert_eq!(Stp258Tokens::total_issuance(DNAR), 350);

			// slashed_amount == amount
			assert_eq!(Stp258Tokens::slash(DNAR, &ALICE, 51), 1);
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 0);
			assert_eq!(Stp258Tokens::total_issuance(DNAR), 300);
		});
}

#[test]
fn update_balance_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob_n_serper_n_settpay()
		.build()
		.execute_with(|| {
			assert_ok!(Stp258Tokens::update_balance(DNAR, &ALICE, 50));
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 150);
			assert_eq!(Stp258Tokens::total_issuance(DNAR), 450);

			assert_ok!(Stp258Tokens::update_balance(DNAR, &BOB, -50));
			assert_eq!(Stp258Tokens::free_balance(DNAR, &BOB), 50);
			assert_eq!(Stp258Tokens::total_issuance(DNAR), 400);

			assert_noop!(Stp258Tokens::update_balance(DNAR, &BOB, -60), Error::<Runtime>::BalanceTooLow);
		});
}

#[test]
fn ensure_can_withdraw_should_work() {
	ExtBuilder::default()
		.one_hundred_for_alice_n_bob_n_serper_n_settpay()
		.build()
		.execute_with(|| {
			assert_noop!(
				Stp258Tokens::ensure_can_withdraw(DNAR, &ALICE, 101),
				Error::<Runtime>::BalanceTooLow
			);

			assert_ok!(Stp258Tokens::ensure_can_withdraw(DNAR, &ALICE, 1));
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 100);
		});
}

#[test]
fn no_op_if_amount_is_zero() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(Stp258Tokens::ensure_can_withdraw(DNAR, &ALICE, 0));
		assert_ok!(Stp258Tokens::transfer(Some(ALICE).into(), BOB, DNAR, 0));
		assert_ok!(Stp258Tokens::transfer(Some(ALICE).into(), ALICE, DNAR, 0));
		assert_ok!(Stp258Tokens::deposit(DNAR, &ALICE, 0));
		assert_ok!(Stp258Tokens::withdraw(DNAR, &ALICE, 0));
		assert_eq!(Stp258Tokens::slash(DNAR, &ALICE, 0), 0);
		assert_eq!(Stp258Tokens::slash(DNAR, &ALICE, 1), 1);
		assert_ok!(Stp258Tokens::update_balance(DNAR, &ALICE, 0));
	});
}

#[test]
fn merge_account_should_work() {
	ExtBuilder::default()
		.balances(vec![(ALICE, DNAR, 100), (ALICE, JUSD, 200 * 1_000)])
		.build()
		.execute_with(|| {
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 100);
			assert_eq!(Stp258Tokens::free_balance(JUSD, &ALICE), 200 * 1_000);
			assert_eq!(Stp258Tokens::free_balance(DNAR, &BOB), 0);

			assert_ok!(Stp258Tokens::reserve(DNAR, &ALICE, 1));
			assert_noop!(
				Stp258Tokens::merge_account(&ALICE, &BOB),
				Error::<Runtime>::StillHasActiveReserved
			);
			Stp258Tokens::unreserve(DNAR, &ALICE, 1);

			assert_ok!(Stp258Tokens::merge_account(&ALICE, &BOB));
			assert_eq!(Stp258Tokens::free_balance(DNAR, &ALICE), 0);
			assert_eq!(Stp258Tokens::free_balance(JUSD, &ALICE), 0 * 1_000);
			assert_eq!(Stp258Tokens::free_balance(DNAR, &BOB), 100);
			assert_eq!(Stp258Tokens::free_balance(JUSD, &BOB), 200 * 1_000);
		});
}

#[test]
fn currency_adapter_ensure_currency_adapter_should_work() {
	ExtBuilder::default()
		.one_hundred_for_treasury_account()
		.build()
		.execute_with(|| {
			assert_eq!(Stp258Tokens::total_issuance(DNAR), 102);
			assert_eq!(Stp258Tokens::total_balance(DNAR, &Treasury::account_id()), 2);
			assert_eq!(Stp258Tokens::total_balance(DNAR, &TREASURY_ACCOUNT), 100);
			assert_eq!(Stp258Tokens::reserved_balance(DNAR, &TREASURY_ACCOUNT), 0);
			assert_eq!(Stp258Tokens::free_balance(DNAR, &TREASURY_ACCOUNT), 100);
			assert_eq!(
				<Runtime as pallet_elections_phragmen::Config>::Currency::total_balance(&TREASURY_ACCOUNT),
				100
			);
			assert_eq!(
				<Runtime as pallet_elections_phragmen::Config>::Currency::can_slash(&TREASURY_ACCOUNT, 10),
				true
			);
			assert_eq!(
				<Runtime as pallet_elections_phragmen::Config>::Currency::total_issuance(),
				102
			);
			assert_eq!(
				<Runtime as pallet_elections_phragmen::Config>::Currency::minimum_balance(),
				2
			);
			assert_eq!(
				<Runtime as pallet_elections_phragmen::Config>::Currency::can_reserve(&TREASURY_ACCOUNT, 5),
				true
			);

			// burn
			let imbalance = <Runtime as pallet_elections_phragmen::Config>::Currency::burn(10);
			assert_eq!(
				<Runtime as pallet_elections_phragmen::Config>::Currency::total_issuance(),
				92
			);
			drop(imbalance);
			assert_eq!(
				<Runtime as pallet_elections_phragmen::Config>::Currency::total_issuance(),
				102
			);

			// issue
			let imbalance = <Runtime as pallet_elections_phragmen::Config>::Currency::issue(20);
			assert_eq!(
				<Runtime as pallet_elections_phragmen::Config>::Currency::total_issuance(),
				122
			);
			drop(imbalance);
			assert_eq!(
				<Runtime as pallet_elections_phragmen::Config>::Currency::total_issuance(),
				102
			);

			// transfer
			assert_eq!(
				<Runtime as pallet_elections_phragmen::Config>::Currency::free_balance(&TREASURY_ACCOUNT),
				100
			);
			assert_ok!(
				<Runtime as pallet_elections_phragmen::Config>::Currency::ensure_can_withdraw(
					&TREASURY_ACCOUNT,
					10,
					WithdrawReasons::TRANSFER,
					0
				)
			);
			assert_ok!(<Runtime as pallet_elections_phragmen::Config>::Currency::transfer(
				&TREASURY_ACCOUNT,
				&ALICE,
				11,
				ExistenceRequirement::KeepAlive
			));
			assert_eq!(
				<Runtime as pallet_elections_phragmen::Config>::Currency::free_balance(&TREASURY_ACCOUNT),
				89
			);

			// deposit
			assert_eq!(
				<Runtime as pallet_elections_phragmen::Config>::Currency::total_issuance(),
				102
			);
			let imbalance = TreasuryCurrencyAdapter::deposit_creating(&TREASURY_ACCOUNT, 11);
			assert_eq!(
				<Runtime as pallet_elections_phragmen::Config>::Currency::free_balance(&TREASURY_ACCOUNT),
				100
			);
			assert_eq!(
				<Runtime as pallet_elections_phragmen::Config>::Currency::total_issuance(),
				102
			);
			drop(imbalance);
			assert_eq!(
				<Runtime as pallet_elections_phragmen::Config>::Currency::free_balance(&TREASURY_ACCOUNT),
				100
			);
			assert_eq!(
				<Runtime as pallet_elections_phragmen::Config>::Currency::total_issuance(),
				113
			);

			// withdraw
			let imbalance = <Runtime as pallet_elections_phragmen::Config>::Currency::withdraw(
				&TREASURY_ACCOUNT,
				10,
				WithdrawReasons::TRANSFER,
				ExistenceRequirement::KeepAlive,
			);
			assert_eq!(
				<Runtime as pallet_elections_phragmen::Config>::Currency::free_balance(&TREASURY_ACCOUNT),
				90
			);
			assert_eq!(
				<Runtime as pallet_elections_phragmen::Config>::Currency::total_issuance(),
				113
			);
			drop(imbalance);
			assert_eq!(
				<Runtime as pallet_elections_phragmen::Config>::Currency::free_balance(&TREASURY_ACCOUNT),
				90
			);
			assert_eq!(
				<Runtime as pallet_elections_phragmen::Config>::Currency::total_issuance(),
				103
			);
		});
}

#[test]
fn currency_adapter_burn_must_work() {
	ExtBuilder::default()
		.one_hundred_for_treasury_account()
		.build()
		.execute_with(|| {
			let init_total_issuance = TreasuryCurrencyAdapter::total_issuance();
			let imbalance = TreasuryCurrencyAdapter::burn(10);
			assert_eq!(TreasuryCurrencyAdapter::total_issuance(), init_total_issuance - 10);
			drop(imbalance);
			assert_eq!(TreasuryCurrencyAdapter::total_issuance(), init_total_issuance);
		});
}

#[test]
fn currency_adapter_reserving_balance_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		let _ = TreasuryCurrencyAdapter::deposit_creating(&TREASURY_ACCOUNT, 111);

		assert_eq!(TreasuryCurrencyAdapter::total_balance(&TREASURY_ACCOUNT), 111);
		assert_eq!(TreasuryCurrencyAdapter::free_balance(&TREASURY_ACCOUNT), 111);
		assert_eq!(TreasuryCurrencyAdapter::reserved_balance(&TREASURY_ACCOUNT), 0);

		assert_ok!(TreasuryCurrencyAdapter::reserve(&TREASURY_ACCOUNT, 69));

		assert_eq!(TreasuryCurrencyAdapter::total_balance(&TREASURY_ACCOUNT), 111);
		assert_eq!(TreasuryCurrencyAdapter::free_balance(&TREASURY_ACCOUNT), 42);
		assert_eq!(TreasuryCurrencyAdapter::reserved_balance(&TREASURY_ACCOUNT), 69);
	});
}

#[test]
fn currency_adapter_balance_transfer_when_reserved_should_not_work() {
	ExtBuilder::default().build().execute_with(|| {
		let _ = TreasuryCurrencyAdapter::deposit_creating(&TREASURY_ACCOUNT, 111);
		assert_ok!(TreasuryCurrencyAdapter::reserve(&TREASURY_ACCOUNT, 69));
		assert_noop!(
			TreasuryCurrencyAdapter::transfer(&TREASURY_ACCOUNT, &ALICE, 69, ExistenceRequirement::AllowDeath),
			Error::<Runtime>::BalanceTooLow,
		);
	});
}

#[test]
fn currency_adapter_deducting_balance_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		let _ = TreasuryCurrencyAdapter::deposit_creating(&TREASURY_ACCOUNT, 111);
		assert_ok!(TreasuryCurrencyAdapter::reserve(&TREASURY_ACCOUNT, 69));
		assert_eq!(TreasuryCurrencyAdapter::free_balance(&TREASURY_ACCOUNT), 42);
	});
}

#[test]
fn currency_adapter_refunding_balance_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		let _ = TreasuryCurrencyAdapter::deposit_creating(&TREASURY_ACCOUNT, 42);
		Stp258Tokens::set_reserved_balance(DNAR, &TREASURY_ACCOUNT, 69);
		TreasuryCurrencyAdapter::unreserve(&TREASURY_ACCOUNT, 69);
		assert_eq!(TreasuryCurrencyAdapter::free_balance(&TREASURY_ACCOUNT), 111);
		assert_eq!(TreasuryCurrencyAdapter::reserved_balance(&TREASURY_ACCOUNT), 0);
	});
}

#[test]
fn currency_adapter_slashing_balance_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		let _ = TreasuryCurrencyAdapter::deposit_creating(&TREASURY_ACCOUNT, 111);
		assert_ok!(TreasuryCurrencyAdapter::reserve(&TREASURY_ACCOUNT, 69));
		assert!(TreasuryCurrencyAdapter::slash(&TREASURY_ACCOUNT, 69).1.is_zero());
		assert_eq!(TreasuryCurrencyAdapter::free_balance(&TREASURY_ACCOUNT), 0);
		assert_eq!(TreasuryCurrencyAdapter::reserved_balance(&TREASURY_ACCOUNT), 42);
		assert_eq!(TreasuryCurrencyAdapter::total_issuance(), 42);
	});
}

#[test]
fn currency_adapter_slashing_incomplete_balance_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		let _ = TreasuryCurrencyAdapter::deposit_creating(&TREASURY_ACCOUNT, 42);
		assert_ok!(TreasuryCurrencyAdapter::reserve(&TREASURY_ACCOUNT, 21));
		assert_eq!(TreasuryCurrencyAdapter::slash(&TREASURY_ACCOUNT, 69).1, 27);
		assert_eq!(TreasuryCurrencyAdapter::free_balance(&TREASURY_ACCOUNT), 0);
		assert_eq!(TreasuryCurrencyAdapter::reserved_balance(&TREASURY_ACCOUNT), 0);
		assert_eq!(TreasuryCurrencyAdapter::total_issuance(), 0);
	});
}

#[test]
fn currency_adapter_basic_locking_should_work() {
	ExtBuilder::default()
		.one_hundred_for_treasury_account()
		.build()
		.execute_with(|| {
			assert_eq!(TreasuryCurrencyAdapter::free_balance(&TREASURY_ACCOUNT), 100);
			TreasuryCurrencyAdapter::set_lock(ID_1, &TREASURY_ACCOUNT, 91, WithdrawReasons::all());
			assert_noop!(
				TreasuryCurrencyAdapter::transfer(&TREASURY_ACCOUNT, &ALICE, 10, ExistenceRequirement::AllowDeath),
				Error::<Runtime>::LiquidityRestrictions
			);
		});
}

#[test]
fn currency_adapter_partial_locking_should_work() {
	ExtBuilder::default()
		.one_hundred_for_treasury_account()
		.build()
		.execute_with(|| {
			TreasuryCurrencyAdapter::set_lock(ID_1, &TREASURY_ACCOUNT, 5, WithdrawReasons::all());
			assert_ok!(TreasuryCurrencyAdapter::transfer(
				&TREASURY_ACCOUNT,
				&ALICE,
				1,
				ExistenceRequirement::AllowDeath
			));
		});
}

#[test]
fn currency_adapter_lock_removal_should_work() {
	ExtBuilder::default()
		.one_hundred_for_treasury_account()
		.build()
		.execute_with(|| {
			TreasuryCurrencyAdapter::set_lock(ID_1, &TREASURY_ACCOUNT, u64::max_value(), WithdrawReasons::all());
			TreasuryCurrencyAdapter::remove_lock(ID_1, &TREASURY_ACCOUNT);
			assert_ok!(TreasuryCurrencyAdapter::transfer(
				&TREASURY_ACCOUNT,
				&ALICE,
				1,
				ExistenceRequirement::AllowDeath
			));
		});
}

#[test]
fn currency_adapter_lock_replacement_should_work() {
	ExtBuilder::default()
		.one_hundred_for_treasury_account()
		.build()
		.execute_with(|| {
			TreasuryCurrencyAdapter::set_lock(ID_1, &TREASURY_ACCOUNT, u64::max_value(), WithdrawReasons::all());
			TreasuryCurrencyAdapter::set_lock(ID_1, &TREASURY_ACCOUNT, 5, WithdrawReasons::all());
			assert_ok!(TreasuryCurrencyAdapter::transfer(
				&TREASURY_ACCOUNT,
				&ALICE,
				1,
				ExistenceRequirement::AllowDeath
			));
		});
}

#[test]
fn currency_adapter_double_locking_should_work() {
	ExtBuilder::default()
		.one_hundred_for_treasury_account()
		.build()
		.execute_with(|| {
			TreasuryCurrencyAdapter::set_lock(ID_1, &TREASURY_ACCOUNT, 5, WithdrawReasons::empty());
			TreasuryCurrencyAdapter::set_lock(ID_2, &TREASURY_ACCOUNT, 5, WithdrawReasons::all());
			assert_ok!(TreasuryCurrencyAdapter::transfer(
				&TREASURY_ACCOUNT,
				&ALICE,
				1,
				ExistenceRequirement::AllowDeath
			));
		});
}

#[test]
fn currency_adapter_combination_locking_should_work() {
	ExtBuilder::default()
		.one_hundred_for_treasury_account()
		.build()
		.execute_with(|| {
			// withdrawReasons not work
			TreasuryCurrencyAdapter::set_lock(ID_1, &TREASURY_ACCOUNT, u64::max_value(), WithdrawReasons::empty());
			TreasuryCurrencyAdapter::set_lock(ID_2, &TREASURY_ACCOUNT, 0, WithdrawReasons::all());
			assert_noop!(
				TreasuryCurrencyAdapter::transfer(&TREASURY_ACCOUNT, &ALICE, 1, ExistenceRequirement::AllowDeath),
				Error::<Runtime>::LiquidityRestrictions
			);
		});
}

#[test]
fn currency_adapter_lock_value_extension_should_work() {
	ExtBuilder::default()
		.one_hundred_for_treasury_account()
		.build()
		.execute_with(|| {
			TreasuryCurrencyAdapter::set_lock(ID_1, &TREASURY_ACCOUNT, 100, WithdrawReasons::all());
			assert_noop!(
				TreasuryCurrencyAdapter::transfer(&TREASURY_ACCOUNT, &ALICE, 6, ExistenceRequirement::AllowDeath),
				Error::<Runtime>::LiquidityRestrictions
			);
			TreasuryCurrencyAdapter::extend_lock(ID_1, &TREASURY_ACCOUNT, 2, WithdrawReasons::all());
			assert_noop!(
				TreasuryCurrencyAdapter::transfer(&TREASURY_ACCOUNT, &ALICE, 6, ExistenceRequirement::AllowDeath),
				Error::<Runtime>::LiquidityRestrictions
			);
			TreasuryCurrencyAdapter::extend_lock(ID_1, &TREASURY_ACCOUNT, 8, WithdrawReasons::all());
			assert_noop!(
				TreasuryCurrencyAdapter::transfer(&TREASURY_ACCOUNT, &ALICE, 3, ExistenceRequirement::AllowDeath),
				Error::<Runtime>::LiquidityRestrictions
			);
		});
}

#[test]
fn currency_adapter_lock_block_number_extension_should_work() {
	ExtBuilder::default()
		.one_hundred_for_treasury_account()
		.build()
		.execute_with(|| {
			TreasuryCurrencyAdapter::set_lock(ID_1, &TREASURY_ACCOUNT, 200, WithdrawReasons::all());
			assert_noop!(
				TreasuryCurrencyAdapter::transfer(&TREASURY_ACCOUNT, &ALICE, 6, ExistenceRequirement::AllowDeath),
				Error::<Runtime>::LiquidityRestrictions
			);
			TreasuryCurrencyAdapter::extend_lock(ID_1, &TREASURY_ACCOUNT, 90, WithdrawReasons::all());
			assert_noop!(
				TreasuryCurrencyAdapter::transfer(&TREASURY_ACCOUNT, &ALICE, 6, ExistenceRequirement::AllowDeath),
				Error::<Runtime>::LiquidityRestrictions
			);
			System::set_block_number(2);
			TreasuryCurrencyAdapter::extend_lock(ID_1, &TREASURY_ACCOUNT, 90, WithdrawReasons::all());
			assert_noop!(
				TreasuryCurrencyAdapter::transfer(&TREASURY_ACCOUNT, &ALICE, 3, ExistenceRequirement::AllowDeath),
				Error::<Runtime>::LiquidityRestrictions
			);
		});
}

#[test]
fn currency_adapter_lock_reasons_extension_should_work() {
	ExtBuilder::default()
		.one_hundred_for_treasury_account()
		.build()
		.execute_with(|| {
			TreasuryCurrencyAdapter::set_lock(ID_1, &TREASURY_ACCOUNT, 90, WithdrawReasons::TRANSFER);
			assert_noop!(
				TreasuryCurrencyAdapter::transfer(&TREASURY_ACCOUNT, &ALICE, 11, ExistenceRequirement::AllowDeath),
				Error::<Runtime>::LiquidityRestrictions
			);
			TreasuryCurrencyAdapter::extend_lock(ID_1, &TREASURY_ACCOUNT, 90, WithdrawReasons::empty());
			assert_noop!(
				TreasuryCurrencyAdapter::transfer(&TREASURY_ACCOUNT, &ALICE, 11, ExistenceRequirement::AllowDeath),
				Error::<Runtime>::LiquidityRestrictions
			);
			TreasuryCurrencyAdapter::extend_lock(ID_1, &TREASURY_ACCOUNT, 90, WithdrawReasons::RESERVE);
			assert_noop!(
				TreasuryCurrencyAdapter::transfer(&TREASURY_ACCOUNT, &ALICE, 11, ExistenceRequirement::AllowDeath),
				Error::<Runtime>::LiquidityRestrictions
			);
		});
}

#[test]
fn currency_adapter_reward_should_work() {
	ExtBuilder::default()
		.one_hundred_for_treasury_account()
		.build()
		.execute_with(|| {
			assert_eq!(TreasuryCurrencyAdapter::total_issuance(), 102);
			assert_eq!(TreasuryCurrencyAdapter::total_balance(&TREASURY_ACCOUNT), 100);
			assert_eq!(TreasuryCurrencyAdapter::total_balance(&Treasury::account_id()), 2);
			assert_ok!(TreasuryCurrencyAdapter::deposit_into_existing(&TREASURY_ACCOUNT, 10).map(drop));
			assert_eq!(TreasuryCurrencyAdapter::total_balance(&TREASURY_ACCOUNT), 110);
			assert_eq!(TreasuryCurrencyAdapter::total_issuance(), 112);
		});
}

#[test]
fn currency_adapter_slashing_reserved_balance_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		let _ = TreasuryCurrencyAdapter::deposit_creating(&TREASURY_ACCOUNT, 111);
		assert_ok!(TreasuryCurrencyAdapter::reserve(&TREASURY_ACCOUNT, 111));
		assert_eq!(TreasuryCurrencyAdapter::slash_reserved(&TREASURY_ACCOUNT, 42).1, 0);
		assert_eq!(TreasuryCurrencyAdapter::reserved_balance(&TREASURY_ACCOUNT), 69);
		assert_eq!(TreasuryCurrencyAdapter::free_balance(&TREASURY_ACCOUNT), 0);
		assert_eq!(TreasuryCurrencyAdapter::total_issuance(), 69);
	});
}

#[test]
fn currency_adapter_slashing_incomplete_reserved_balance_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		let _ = TreasuryCurrencyAdapter::deposit_creating(&TREASURY_ACCOUNT, 111);
		assert_ok!(TreasuryCurrencyAdapter::reserve(&TREASURY_ACCOUNT, 42));
		assert_eq!(TreasuryCurrencyAdapter::slash_reserved(&TREASURY_ACCOUNT, 69).1, 27);
		assert_eq!(TreasuryCurrencyAdapter::free_balance(&TREASURY_ACCOUNT), 69);
		assert_eq!(TreasuryCurrencyAdapter::reserved_balance(&TREASURY_ACCOUNT), 0);
		assert_eq!(TreasuryCurrencyAdapter::total_issuance(), 69);
	});
}

#[test]
fn currency_adapter_repatriating_reserved_balance_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		let _ = TreasuryCurrencyAdapter::deposit_creating(&TREASURY_ACCOUNT, 110);
		let _ = TreasuryCurrencyAdapter::deposit_creating(&ALICE, 2);
		assert_ok!(TreasuryCurrencyAdapter::reserve(&TREASURY_ACCOUNT, 110));
		assert_ok!(
			TreasuryCurrencyAdapter::repatriate_reserved(&TREASURY_ACCOUNT, &ALICE, 41, Status::Free),
			0
		);
		assert_eq!(TreasuryCurrencyAdapter::reserved_balance(&TREASURY_ACCOUNT), 69);
		assert_eq!(TreasuryCurrencyAdapter::free_balance(&TREASURY_ACCOUNT), 0);
		assert_eq!(TreasuryCurrencyAdapter::reserved_balance(&ALICE), 0);
		assert_eq!(TreasuryCurrencyAdapter::free_balance(&ALICE), 43);
	});
}

#[test]
fn currency_adapter_transferring_reserved_balance_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		let _ = TreasuryCurrencyAdapter::deposit_creating(&TREASURY_ACCOUNT, 110);
		let _ = TreasuryCurrencyAdapter::deposit_creating(&ALICE, 2);
		assert_ok!(TreasuryCurrencyAdapter::reserve(&TREASURY_ACCOUNT, 110));
		assert_ok!(
			TreasuryCurrencyAdapter::repatriate_reserved(&TREASURY_ACCOUNT, &ALICE, 41, Status::Reserved),
			0
		);
		assert_eq!(TreasuryCurrencyAdapter::reserved_balance(&TREASURY_ACCOUNT), 69);
		assert_eq!(TreasuryCurrencyAdapter::free_balance(&TREASURY_ACCOUNT), 0);
		assert_eq!(TreasuryCurrencyAdapter::reserved_balance(&ALICE), 41);
		assert_eq!(TreasuryCurrencyAdapter::free_balance(&ALICE), 2);
	});
}

#[test]
fn currency_adapter_transferring_reserved_balance_to_nonexistent_should_fail() {
	ExtBuilder::default().build().execute_with(|| {
		let _ = TreasuryCurrencyAdapter::deposit_creating(&TREASURY_ACCOUNT, 111);
		assert_ok!(TreasuryCurrencyAdapter::reserve(&TREASURY_ACCOUNT, 111));
		assert_ok!(TreasuryCurrencyAdapter::repatriate_reserved(
			&TREASURY_ACCOUNT,
			&ALICE,
			42,
			Status::Free
		));
	});
}

#[test]
fn currency_adapter_transferring_incomplete_reserved_balance_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		let _ = TreasuryCurrencyAdapter::deposit_creating(&TREASURY_ACCOUNT, 110);
		let _ = TreasuryCurrencyAdapter::deposit_creating(&ALICE, 2);
		assert_ok!(TreasuryCurrencyAdapter::reserve(&TREASURY_ACCOUNT, 41));
		assert_ok!(
			TreasuryCurrencyAdapter::repatriate_reserved(&TREASURY_ACCOUNT, &ALICE, 69, Status::Free),
			28
		);
		assert_eq!(TreasuryCurrencyAdapter::reserved_balance(&TREASURY_ACCOUNT), 0);
		assert_eq!(TreasuryCurrencyAdapter::free_balance(&TREASURY_ACCOUNT), 69);
		assert_eq!(TreasuryCurrencyAdapter::reserved_balance(&ALICE), 0);
		assert_eq!(TreasuryCurrencyAdapter::free_balance(&ALICE), 43);
	});
}

#[test]
fn currency_adapter_transferring_too_high_value_should_not_panic() {
	ExtBuilder::default().build().execute_with(|| {
		TreasuryCurrencyAdapter::make_free_balance_be(&TREASURY_ACCOUNT, u64::max_value());
		TreasuryCurrencyAdapter::make_free_balance_be(&ALICE, 2);

		assert_noop!(
			TreasuryCurrencyAdapter::transfer(
				&TREASURY_ACCOUNT,
				&ALICE,
				u64::max_value(),
				ExistenceRequirement::AllowDeath
			),
			Error::<Runtime>::BalanceOverflow,
		);

		assert_eq!(
			TreasuryCurrencyAdapter::free_balance(&TREASURY_ACCOUNT),
			u64::max_value()
		);
		assert_eq!(TreasuryCurrencyAdapter::free_balance(&ALICE), 2);
	});
}
