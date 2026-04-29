use anyhow::Result;
use reqwest::Client;

// Hardcoded well-known selectors as fallback
fn known_selector(sel: &str) -> Option<&'static str> {
    match sel {
        "a9059cbb" => Some("transfer(address,uint256)"),
        "23b872dd" => Some("transferFrom(address,address,uint256)"),
        "095ea7b3" => Some("approve(address,uint256)"),
        "70a08231" => Some("balanceOf(address)"),
        "18160ddd" => Some("totalSupply()"),
        "dd62ed3e" => Some("allowance(address,address)"),
        "7ff36ab5" => Some("swapExactETHForTokens(uint256,address[],address,uint256)"),
        "38ed1739" => Some("swapExactTokensForTokens(uint256,uint256,address[],address,uint256)"),
        "414bf389" => Some("exactInputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))"),
        "e8e33700" => Some("addLiquidity(address,address,uint256,uint256,uint256,uint256,address,uint256)"),
        "f305d719" => Some("addLiquidityETH(address,uint256,uint256,uint256,address,uint256)"),
        "a22cb465" => Some("setApprovalForAll(address,bool)"),
        "6352211e" => Some("ownerOf(uint256)"),
        "42842e0e" => Some("safeTransferFrom(address,address,uint256)"),
        _ => None,
    }
}

pub async fn lookup_4byte(selector: &str) -> Result<Option<String>> {
    let sel = selector.trim_start_matches("0x");

    // check known selectors first
    if let Some(sig) = known_selector(sel) {
        return Ok(Some(sig.to_string()));
    }

    let url = format!(
        "https://www.4byte.directory/api/v1/signatures/?hex_signature=0x{}&ordering=id",
        sel
    );

    let resp: serde_json::Value = Client::new()
        .get(&url)
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await?
        .json()
        .await?;

    // print raw for debugging
    if let Some(results) = resp["results"].as_array() {
        eprintln!("[4byte] {} results for 0x{}:", results.len(), sel);
        for r in results {
            eprintln!("  id={} sig={}", r["id"], r["text_signature"]);
        }
    }

    let results = match resp["results"].as_array() {
        Some(r) if !r.is_empty() => r,
        _ => return Ok(None),
    };

    // ordering=id ASC = oldest registration = most canonical
    // take the first result that looks like a real function sig
    let best = results
        .iter()
        .filter_map(|r| r["text_signature"].as_str())
        .find(|s| s.contains('(') && s.contains(')') && s.len() < 80);

    Ok(best.map(|s| s.to_string()))
}
