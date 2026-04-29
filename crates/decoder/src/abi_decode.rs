use alloy::dyn_abi::{DynSolType, DynSolValue};
use anyhow::Result;
use evmdecode_core::types::{DecodedCall, DecodedParam};

fn sol_type_name(v: &DynSolValue) -> String {
    v.sol_type_name()
        .map(|c| c.to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

pub fn decode_calldata(calldata: &[u8], sig: &str) -> Result<DecodedCall> {
    if calldata.len() < 4 {
        anyhow::bail!("calldata too short (less than 4 bytes)");
    }

    let selector = hex::encode(&calldata[..4]);
    let params_bytes = &calldata[4..];

    let params_str = sig
        .find('(')
        .map(|i| &sig[i..])
        .unwrap_or("()");

    let ty: DynSolType = params_str.parse()?;
    let decoded = ty.abi_decode_sequence(params_bytes)?;

    let params = match decoded {
        DynSolValue::Tuple(vals) => vals
            .iter()
            .map(|v| DecodedParam {
                name: None,
                kind: sol_type_name(v),
                value: format!("{v:?}"),
            })
            .collect(),
        other => vec![DecodedParam {
            name: None,
            kind: sol_type_name(&other),
            value: format!("{other:?}"),
        }],
    };

    Ok(DecodedCall {
        selector,
        function_sig: Some(sig.to_string()),
        params,
    })
}
