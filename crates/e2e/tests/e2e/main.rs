// <crate>/tests signals to Cargo that files inside of it are integration tests.
// Integration tests are compiled into separate binaries which is slow. To avoid
// this we create one integration test here and in this test we include all the
// tests we want to run.

// Each of the following modules contains tests.
mod app_data;
mod app_data_signer;
mod banned_users;
mod buffers;
mod cow_amm;
mod database;
mod eth_integration;
mod eth_safe;
mod ethflow;
mod flashloans;
mod hooks;
mod jit_orders;
mod limit_orders;
mod liquidity;
mod order_cancellation;
mod partial_fill;
mod partially_fillable_balance;
mod partially_fillable_pool;
mod place_order_with_quote;
mod protocol_fee;
mod quote_verification;
mod quoting;
mod refunder;
mod replace_order;
mod smart_contract_orders;
mod solver_competition;
mod solver_participation_guard;
mod submission;
mod tracking_insufficient_funds;
mod uncovered_order;
mod univ2;
mod vault_balances;
