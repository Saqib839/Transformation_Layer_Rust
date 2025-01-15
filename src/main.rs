use reqstreams::run_substream;
use module_map::decode_with_module;

mod pb;
mod module_map; 

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let endpoint_url = "https://mainnet.eth.streamingfast.io:443".to_string();
    let package_file = "https://github.com/streamingfast/substreams-uniswap-v3/releases/download/v0.2.10/substreams-uniswap-v3-v0.2.10.spkg";
    let module_name = "uni_v0_2_9:map_tokens_whitelist_pools"; // Change this dynamically as needed
    let block_range = Some("21335037:21335038".to_string());

    let results = run_substream(endpoint_url, package_file, module_name, block_range).await?;

    for vec_bytes in results {
        // Use the decode_with_module helper to decode dynamically
        match decode_with_module(module_name, &vec_bytes) {
            Ok(decoded) => {
                println!("{:#?}", decoded);
            }
            Err(err) => {
                eprintln!("Failed to decode module {}: {:?}", module_name, err);
            }
        }
    }

    Ok(())
}
