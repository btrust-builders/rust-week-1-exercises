use std::collections::HashMap;

pub const MINING_REWARD: f64 = 3.125;

pub const CURRENT_BLOCK_HEIGHT: u64 = 840000;

pub const BTC_TO_SATS: u64 = 100_000_000;

#[derive(Debug, Clone, PartialEq)]
pub struct Utxo {
    pub txid: String,
    pub vout: u32,
    pub value: u64,
}

/// Calculate the total Bitcoin reward for a given number of mined blocks.
pub fn calculate_total_reward(blocks_mined: u64) -> f64 {
    blocks_mined as f64 * MINING_REWARD
}

/// Return true if the transaction fee is between 0.00001 and 0.01 BTC.
pub fn is_valid_tx_fee(fee: f64) -> bool {
    fee >= 0.00001 && fee <= 0.01
}

/// Return true if the wallet balance is greater than 50.0 BTC.
pub fn is_large_balance(balance: f64) -> bool {
    balance > 50.0
}

/// Return the priority of a transaction ("high", "medium", "low") based on fee rate.
pub fn tx_priority(size_bytes: u64, fee_btc: f64) -> &'static str {
    let fee_rate = fee_btc / size_bytes as f64;
    if fee_rate > 0.00005 {
        "high"
    } else if fee_rate > 0.00001 {
        "medium"
    } else {
        "low"
    }
}

/// Return true if the network string equals "mainnet" (case-insensitive).
pub fn is_mainnet(network: &str) -> bool {
    network.to_lowercase() == "mainnet"
}

/// Return true if value is in the inclusive range 100..=200.
pub fn is_in_range(value: i64) -> bool {
    (100..=200).contains(&value)
}

/// Return true if both references point to the exact same object in memory.
pub fn is_same_wallet<T>(wallet1: &T, wallet2: &T) -> bool {
    std::ptr::eq(wallet1, wallet2)
}

pub fn normalize_address(address: &str) -> String {
    address.trim().to_lowercase()
}

/// Append a new UTXO to the list and return the updated list.
pub fn add_utxo(mut utxos: Vec<Utxo>, new_utxo: Utxo) -> Vec<Utxo> {
    utxos.push(new_utxo);
    utxos
}

/// Find the first transaction with a fee greater than 0.005 BTC.
pub fn find_high_fee(fee_list: &[f64]) -> Option<(usize, f64)> {
    for (index, fee) in fee_list.iter().enumerate() {
        if *fee > 0.005 {
            return Some((index, *fee));
        }
    }
    None
}

/// Return basic wallet details as a tuple of (name, balance).
pub fn get_wallet_details() -> (String, f64) {
    ("satoshi_wallet".to_string(), 50.0)
}

/// Get the status of a transaction from the mempool or "not found".
pub fn get_tx_status(tx_pool: &HashMap<String, String>, txid: &str) -> String {
    tx_pool
        .get(txid)
        .unwrap_or(&"not found".to_string())
        .clone()
}

/// Destructure wallet_info and format a status string.
pub fn unpack_wallet_info(wallet_info: (String, f64)) -> String {
    let (name, balance) = wallet_info;
    format!("Wallet {} has balance: {} BTC", name, balance)
}

/// Convert BTC to satoshis (1 BTC = 100,000,000 sats).
pub fn calculate_sats(btc: f64) -> u64 {
    (btc * BTC_TO_SATS as f64) as u64
}

/// Generate a mock Bitcoin address of length 32 with the given prefix.
pub fn generate_address(prefix: &str) -> String {
    let chars = "abcdefghijklmnopqrstuvwxyz0123456789";
    let suffix_len = 32 - prefix.len();
    let mut result = String::from(prefix);

    for i in 0..suffix_len {
        let idx = (i * 7 + prefix.len()) % chars.len();
        result.push(chars.chars().nth(idx).unwrap());
    }
    result
}

/// Validate a Bitcoin block height. Returns (is_valid, message).
pub fn validate_block_height(height: i64) -> (bool, String) {
      if height < 0 {
        (false, "Invalid: negative block height".to_string())
    } else if height > 800_000 {
        (false, "Invalid: unrealistic block height".to_string())
    } else {
        (true, "Valid block height".to_string())
    }
}

/// Compute the block reward (in sats) for each block height based on the halving schedule.
pub fn halving_schedule(blocks: &[u64]) -> HashMap<u64, u64> {
    let base = 50 * BTC_TO_SATS;
    let mut result = HashMap::new();

    for block in blocks {
        let halvings = block / 210_000;
        let reward = base >> halvings;
        result.insert(*block, reward);
    }

    result
}

/// Find the UTXO with the smallest value that meets or exceeds target.
pub fn find_utxo_with_min_value(utxos: &[Utxo], target: u64) -> Option<Utxo> {
    let mut best: Option<Utxo> = None;

    for utxo in utxos {
        if utxo.value >= target {
            match &best {
                None => best = Some(utxo.clone()),
                Some(current_best) => {
                    if utxo.value < current_best.value {
                        best = Some(utxo.clone());
                    }
                }
            }
        }
    }

    best
}

/// Create a UTXO map from txid, vout, and arbitrary extra string fields.
pub fn create_utxo(
    txid: &str,
    vout: u32,
    extra: HashMap<String, String>,
) -> HashMap<String, String> {
    let mut map = HashMap::new();

    map.insert("txid".to_string(), txid.to_string());
    map.insert("vout".to_string(), vout.to_string());

    map.extend(extra);

    map
}

// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
     if raw_tx_hex.len() < 8 {
        return Err("Transaction data too short".to_string());
    }

    let version_hex = &raw_tx_hex[0..8];

    let bytes = match hex::decode(version_hex) {
        Ok(b) => b,
        Err(_) => return Err("Hex decode error".to_string()),
    };

    if bytes.len() != 4 {
        return Err("Hex decode error".to_string());
    }

    // little-endian conversion (Bitcoin standard)
    let version = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);

    Ok(version)
}
