use evmdecode_decoder::{abi_decode::decode_calldata, selector::lookup_4byte};
use evmdecode_render::pretty::print_decoded_call;
use evmdecode_rpc::{fetcher::fetch_tx, provider::build_provider};

pub async fn run(tx: &str, rpc: &str, _abi: Option<&str>, _output: &str) -> anyhow::Result<()> {
    let provider = build_provider(rpc)?;
    let raw = fetch_tx(&provider, tx).await?;

    println!("tx      : {}", raw.hash);
    println!("from    : {}", raw.from);
    println!("to      : {}", raw.to.as_deref().unwrap_or("contract creation"));
    println!("value   : {} wei", raw.value);
    println!("gas     : {}", raw.gas);

    let calldata = hex::decode(&raw.input)?;
    if calldata.len() < 4 {
        println!("no calldata — plain ETH transfer");
        return Ok(());
    }

    let selector = format!("0x{}", hex::encode(&calldata[..4]));
    println!("selector: {}", selector);

    match lookup_4byte(&selector).await? {
        None => println!("signature: unknown (not in 4byte.directory)"),
        Some(sig) => {
            println!("signature: {}", sig);
            match decode_calldata(&calldata, &sig) {
                Ok(decoded) => print_decoded_call(&decoded),
                Err(e) => println!("decode error: {e}"),
            }
        }
    }

    Ok(())
}
