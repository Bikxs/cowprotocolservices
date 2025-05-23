use {
    crate::{
        domain::liquidity::{
            self,
            zeroex::{self, Amounts, Order, ZeroExSignature},
        },
        infra::{self, Ethereum},
    },
    anyhow::anyhow,
    ethrpc::block_stream::CurrentBlockWatcher,
    shared::{
        http_client::HttpClientFactory,
        price_estimation::gas::GAS_PER_ZEROEX_ORDER,
        zeroex_api::DefaultZeroExApi,
    },
    solver::{
        liquidity::{LimitOrder, zeroex::ZeroExLiquidity},
        liquidity_collector::LiquidityCollecting,
    },
    std::sync::Arc,
};

pub fn to_domain(
    id: liquidity::Id,
    limit_order: LimitOrder,
) -> anyhow::Result<liquidity::Liquidity> {
    // `order` and `contract` should be provided somehow through the `LimitOrder`
    // struct. Currently, it's not possible to add 0x-specific fields right to
    // the `solver::LimitOrder` since it's used with different settlement
    // handlers. One of the options to address it: to use a separate
    // `solver::Liquidity` enum value for 0x liquidity.
    let handler = limit_order
        .settlement_handling
        .as_any()
        .downcast_ref::<solver::liquidity::zeroex::OrderSettlementHandler>()
        .ok_or(anyhow!("not a zeroex::OrderSettlementHandler"))?
        .clone();

    let signature = ZeroExSignature {
        r: handler.order_record.order().signature.r,
        s: handler.order_record.order().signature.s,
        v: handler.order_record.order().signature.v,
        signature_type: handler.order_record.order().signature.signature_type,
    };

    let order = Order {
        maker: handler.order_record.order().maker,
        taker: handler.order_record.order().taker,
        sender: handler.order_record.order().sender,
        maker_token: handler.order_record.order().maker_token,
        taker_token: handler.order_record.order().taker_token,
        amounts: Amounts {
            maker: handler.order_record.order().maker_amount,
            taker: handler.order_record.order().taker_amount,
        },
        taker_token_fee_amount: handler.order_record.order().taker_token_fee_amount,
        fee_recipient: handler.order_record.order().fee_recipient,
        pool: handler.order_record.order().pool,
        expiry: handler.order_record.order().expiry,
        salt: handler.order_record.order().salt,
        signature,
    };

    let domain = zeroex::LimitOrder {
        order,
        fillable: Amounts {
            maker: limit_order.sell_amount.as_u128(),
            taker: limit_order.buy_amount.as_u128(),
        },
        zeroex: handler.zeroex.clone(),
    };

    Ok(liquidity::Liquidity {
        id,
        gas: GAS_PER_ZEROEX_ORDER.into(),
        kind: liquidity::Kind::ZeroEx(domain),
    })
}

pub async fn collector(
    eth: &Ethereum,
    blocks: CurrentBlockWatcher,
    config: &infra::liquidity::config::ZeroEx,
) -> anyhow::Result<Box<dyn LiquidityCollecting>> {
    let eth = eth.with_metric_label("zeroex".into());
    let settlement = eth.contracts().settlement().clone();
    let web3 = settlement.raw_instance().web3().clone();
    let contract = contracts::IZeroEx::deployed(&web3).await?;
    let http_client_factory = &HttpClientFactory::new(&shared::http_client::Arguments {
        http_timeout: config.http_timeout,
    });
    let api = Arc::new(DefaultZeroExApi::new(
        http_client_factory.builder(),
        config.base_url.clone(),
        config.api_key.clone(),
        blocks.clone(),
    )?);
    Ok(Box::new(
        ZeroExLiquidity::new(web3, api, contract, settlement, blocks).await,
    ))
}
