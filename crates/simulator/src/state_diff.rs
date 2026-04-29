use evmdecode_core::types::{StateDiff, StorageChange};
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

pub fn parse_state_diff(trace: &Value) -> Result<Vec<StateDiff>> {
    let mut diffs = Vec::new();

    // prestateTracer diffMode gives { "pre": {...}, "post": {...} }
    let pre  = match trace.get("pre")  { Some(v) => v, None => return Ok(diffs) };
    let post = match trace.get("post") { Some(v) => v, None => return Ok(diffs) };

    // collect all addresses touched in either pre or post
    let mut addresses = std::collections::HashSet::new();
    if let Some(obj) = pre.as_object()  { addresses.extend(obj.keys().cloned()); }
    if let Some(obj) = post.as_object() { addresses.extend(obj.keys().cloned()); }

    for addr in &addresses {
        let pre_entry  = &pre[addr];
        let post_entry = &post[addr];

        // balance before/after
        let balance_before = pre_entry["balance"].as_str().map(|s| s.to_string());
        let balance_after  = post_entry["balance"].as_str().map(|s| s.to_string());

        // storage slot diffs
        let mut storage_changes: HashMap<String, StorageChange> = HashMap::new();

        // collect all slots from pre storage
        if let Some(pre_storage) = pre_entry["storage"].as_object() {
            for (slot, pre_val) in pre_storage {
                let post_val = post_entry["storage"][slot]
                    .as_str()
                    .unwrap_or("0x0000000000000000000000000000000000000000000000000000000000000000");
                let before = pre_val.as_str().unwrap_or("0x00").to_string();
                if before != post_val {
                    storage_changes.insert(slot.clone(), StorageChange {
                        slot: slot.clone(),
                        before,
                        after: post_val.to_string(),
                    });
                }
            }
        }

        // collect slots only in post (new slots written)
        if let Some(post_storage) = post_entry["storage"].as_object() {
            for (slot, post_val) in post_storage {
                if storage_changes.contains_key(slot) { continue; }
                let before = pre_entry["storage"][slot]
                    .as_str()
                    .unwrap_or("0x0000000000000000000000000000000000000000000000000000000000000000")
                    .to_string();
                let after = post_val.as_str().unwrap_or("0x00").to_string();
                if before != after {
                    storage_changes.insert(slot.clone(), StorageChange {
                        slot: slot.clone(),
                        before,
                        after,
                    });
                }
            }
        }

        // only include addresses that actually changed something
        let balance_changed = balance_before != balance_after;
        if !storage_changes.is_empty() || balance_changed {
            diffs.push(StateDiff {
                address: addr.clone(),
                storage_changes,
                balance_before,
                balance_after,
            });
        }
    }

    // sort by address for deterministic output
    diffs.sort_by(|a, b| a.address.cmp(&b.address));
    Ok(diffs)
}
