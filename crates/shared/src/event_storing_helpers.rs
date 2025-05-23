use {
    crate::{
        db_order_conversions::order_kind_into,
        order_quoting::{QuoteData, QuoteSearchParameters, quote_kind_from_signing_scheme},
    },
    anyhow::Result,
    chrono::{DateTime, Utc},
    database::{
        byte_array::ByteArray,
        quotes::{Quote as DbQuote, QuoteSearchParameters as DbQuoteSearchParameters},
    },
    number::conversions::u256_to_big_decimal,
};

pub fn create_quote_row(data: QuoteData) -> Result<DbQuote> {
    Ok(DbQuote {
        id: Default::default(),
        sell_token: ByteArray(data.sell_token.0),
        buy_token: ByteArray(data.buy_token.0),
        sell_amount: u256_to_big_decimal(&data.quoted_sell_amount),
        buy_amount: u256_to_big_decimal(&data.quoted_buy_amount),
        gas_amount: data.fee_parameters.gas_amount,
        gas_price: data.fee_parameters.gas_price,
        sell_token_price: data.fee_parameters.sell_token_price,
        order_kind: order_kind_into(data.kind),
        expiration_timestamp: data.expiration,
        quote_kind: data.quote_kind,
        solver: ByteArray(data.solver.0),
        verified: data.verified,
        metadata: data.metadata.try_into()?,
    })
}

pub fn create_db_search_parameters(
    params: QuoteSearchParameters,
    expiration: DateTime<Utc>,
) -> DbQuoteSearchParameters {
    DbQuoteSearchParameters {
        sell_token: ByteArray(params.sell_token.0),
        buy_token: ByteArray(params.buy_token.0),
        sell_amount_0: u256_to_big_decimal(&params.sell_amount),
        sell_amount_1: u256_to_big_decimal(&(params.sell_amount + params.fee_amount)),
        buy_amount: u256_to_big_decimal(&params.buy_amount),
        kind: order_kind_into(params.kind),
        expiration,
        quote_kind: quote_kind_from_signing_scheme(&params.signing_scheme),
    }
}
