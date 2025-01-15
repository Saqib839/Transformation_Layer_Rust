use crate::pb; // Import the `pb` module
use prost::Message; // Required for decoding
use std::collections::HashMap;
use std::sync::Arc;
use lazy_static::lazy_static;
use anyhow::Error;

// Define a type alias for the decoder function
type DecoderFn = Arc<dyn Fn(&[u8]) -> Result<Box<dyn std::fmt::Debug>, Error> + Sync + Send>;

pub fn decode_with_module(
    module_name: &str,
    data: &[u8],
) -> Result<Box<dyn std::fmt::Debug>, Error> {
    if let Some(decoder) = MODULES.get(module_name) {
        decoder(data)
    } else {
        Err(anyhow::anyhow!("No decoder registered for module: {}", module_name))
    }
}

lazy_static! {
    pub static ref MODULES: HashMap<&'static str, DecoderFn> = {
        let mut map: HashMap<&'static str, DecoderFn> = HashMap::new();

        // Explicitly cast each closure to the `DecoderFn` type
        map.insert(
            "map_pools_created",
            Arc::new(|data: &[u8]| -> Result<Box<dyn std::fmt::Debug>, Error> {
                let decoded = pb::uniswap_types_v1::Pools::decode(data)?; // Decode the Protobuf data
                Ok(Box::new(decoded))
            }) as DecoderFn,
        );
        map.insert(
            "uni_v0_2_9:map_pools_created",
            Arc::new(|data: &[u8]| -> Result<Box<dyn std::fmt::Debug>, Error> {
                let decoded = pb::uniswap_types_v1::Pools::decode(data)?; // Decode the Protobuf data
                Ok(Box::new(decoded))
            }) as DecoderFn,
        );
        map.insert(
            "graph_out",
            Arc::new(|data: &[u8]| -> Result<Box<dyn std::fmt::Debug>, Error> {
                let decoded = pb::substreams_entity_change::EntityChanges::decode(data)?; // Decode the Protobuf data
                Ok(Box::new(decoded))
            }) as DecoderFn,
        );
        map.insert(
            "uni_v0_2_9:graph_out",
            Arc::new(|data: &[u8]| -> Result<Box<dyn std::fmt::Debug>, Error> {
                let decoded = pb::substreams_entity_change::EntityChanges::decode(data)?; // Decode the Protobuf data
                Ok(Box::new(decoded))
            }) as DecoderFn,
        );
        
        map.insert(
            "uni_v0_2_9:map_tokens_whitelist_pools",
            Arc::new(|data: &[u8]| -> Result<Box<dyn std::fmt::Debug>, Error> {
                let decoded = pb::uniswap_types_v1::Erc20Tokens::decode(data)?; // Decode the Protobuf data
                Ok(Box::new(decoded))
            }) as DecoderFn,
        );

        map.insert(
            "map_block_full",
            Arc::new(|data: &[u8]| -> Result<Box<dyn std::fmt::Debug>, Error> {
                let decoded = pb::solana_types_v1::Block::decode(data)?; // Decode the Protobuf data
                Ok(Box::new(decoded))
            }) as DecoderFn,
        );

        map.insert(
            "map_block_without_votes",
            Arc::new(|data: &[u8]| -> Result<Box<dyn std::fmt::Debug>, Error> {
                let decoded = pb::solana_types_v1::Block::decode(data)?; // Decode the Protobuf data
                Ok(Box::new(decoded))
            }) as DecoderFn,
        );

        map.insert(
            "map_block_meta",
            Arc::new(|data: &[u8]| -> Result<Box<dyn std::fmt::Debug>, Error> {
                let decoded = pb::sol_block_v1::BlockMeta::decode(data)?; // Decode the Protobuf data
                Ok(Box::new(decoded))
            }) as DecoderFn,
        );

        map.insert(
            "map_filter_instructions",
            Arc::new(|data: &[u8]| -> Result<Box<dyn std::fmt::Debug>, Error> {
                let decoded = pb::sol_transactions_v1::Instructions::decode(data)?; // Decode the Protobuf data
                Ok(Box::new(decoded))
            }) as DecoderFn,
        );

        map.insert(
            "map_filter_transactions",
            Arc::new(|data: &[u8]| -> Result<Box<dyn std::fmt::Debug>, Error> {
                let decoded = pb::sol_transactions_v1::Transactions::decode(data)?; // Decode the Protobuf data
                Ok(Box::new(decoded))
            }) as DecoderFn,
        );



        // map.insert(
        //     "another_module_event",
        //     Arc::new(|data: &[u8]| -> Result<Box<dyn std::fmt::Debug>, Error> {
        //         let decoded = pb::another_types_v1::AnotherType::decode(data)?; // Decode the Protobuf data
        //         Ok(Box::new(decoded))
        //     }) as DecoderFn,
        // );

        map
    };
}
