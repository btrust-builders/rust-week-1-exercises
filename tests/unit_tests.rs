use rust_week_1_exercises::{
    add_utxo, calculate_sats, calculate_total_reward, create_utxo, extract_tx_version,
    find_high_fee, find_utxo_with_min_value, generate_address, get_tx_status, get_wallet_details,
    halving_schedule, is_in_range, is_large_balance, is_mainnet, is_same_wallet, is_valid_tx_fee,
    normalize_address, tx_priority, unpack_wallet_info, validate_block_height, Utxo, BTC_TO_SATS,
    MINING_REWARD,
};
use std::collections::HashMap;

#[cfg(test)]
mod exercise_tests {
    use super::*;

    #[test]
    fn test_constants_set() {
        assert!(
            MINING_REWARD > 0.0,
            "MINING_REWARD must be set to a positive value"
        );
        assert_eq!(BTC_TO_SATS, 100_000_000);
    }

    #[test]
    fn test_calculate_total_reward() {
        assert_eq!(calculate_total_reward(4), 4.0 * MINING_REWARD);
        assert_eq!(calculate_total_reward(0), 0.0);
    }

    #[test]
    fn test_is_valid_tx_fee() {
        assert!(is_valid_tx_fee(0.0001));
        assert!(is_valid_tx_fee(0.00001));
        assert!(is_valid_tx_fee(0.01));
        assert!(!is_valid_tx_fee(0.0));
        assert!(!is_valid_tx_fee(0.02));
    }

    #[test]
    fn test_is_large_balance() {
        assert!(is_large_balance(75.0));
        assert!(!is_large_balance(25.0));
        assert!(!is_large_balance(50.0));
    }

    #[test]
    fn test_tx_priority() {
        assert_eq!(tx_priority(1000, 0.1), "high");
        assert_eq!(tx_priority(1000, 0.025), "medium");
        assert_eq!(tx_priority(1000, 0.005), "low");
    }

    #[test]
    fn test_is_mainnet() {
        assert!(is_mainnet("mainnet"));
        assert!(is_mainnet("MainNet"));
        assert!(is_mainnet("MAINNET"));
        assert!(!is_mainnet("testnet"));
        assert!(!is_mainnet("signet"));
    }

    #[test]
    fn test_is_in_range() {
        assert!(is_in_range(150));
        assert!(is_in_range(100));
        assert!(is_in_range(200));
        assert!(!is_in_range(99));
        assert!(!is_in_range(201));
    }

    #[test]
    fn test_is_same_wallet() {
        let wallet = String::from("satoshi");
        let other = String::from("satoshi");
        assert!(is_same_wallet(&wallet, &wallet));
        assert!(!is_same_wallet(&wallet, &other));
    }

    #[test]
    fn test_normalize_address() {
        assert_eq!(normalize_address("  BC1qXYZ  "), "bc1qxyz");
        assert_eq!(normalize_address("ABCDEF"), "abcdef");
    }

    #[test]
    fn test_add_utxo() {
        let utxos = vec![Utxo {
            txid: "aaa".into(),
            vout: 0,
            value: 100,
        }];
        let new = Utxo {
            txid: "bbb".into(),
            vout: 1,
            value: 200,
        };
        let result = add_utxo(utxos, new.clone());
        assert_eq!(result.len(), 2);
        assert_eq!(result[1], new);
    }

    #[test]
    fn test_find_high_fee() {
        assert_eq!(find_high_fee(&[0.001, 0.002, 0.01]), Some((2, 0.01)));
        assert_eq!(find_high_fee(&[0.001, 0.002]), None);
        assert_eq!(find_high_fee(&[]), None);
    }

    #[test]
    fn test_get_wallet_details() {
        let (name, balance) = get_wallet_details();
        assert_eq!(name, "satoshi_wallet");
        assert_eq!(balance, 50.0);
    }

    #[test]
    fn test_get_tx_status() {
        let mut pool = HashMap::new();
        pool.insert("tx1".to_string(), "confirmed".to_string());
        assert_eq!(get_tx_status(&pool, "tx1"), "confirmed");
        assert_eq!(get_tx_status(&pool, "missing"), "not found");
    }

    #[test]
    fn test_unpack_wallet_info() {
        let info = ("satoshi_wallet".to_string(), 50.0);
        assert_eq!(
            unpack_wallet_info(info),
            "Wallet satoshi_wallet has balance: 50 BTC"
        );
    }

    #[test]
    fn test_calculate_sats() {
        assert_eq!(calculate_sats(1.0), 100_000_000);
        assert_eq!(calculate_sats(0.5), 50_000_000);
        assert_eq!(calculate_sats(0.0), 0);
    }

    #[test]
    fn test_generate_address() {
        let addr = generate_address("bc1q");
        assert_eq!(addr.len(), 32);
        assert!(addr.starts_with("bc1q"));
        assert!(addr
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
    }

    #[test]
    fn test_validate_block_height_negative() {
        let (ok, msg) = validate_block_height(-1);
        assert!(!ok);
        assert!(msg.to_lowercase().contains("negative"));
    }

    #[test]
    fn test_validate_block_height_too_large() {
        let (ok, msg) = validate_block_height(1_250_000);
        assert!(!ok);
        assert!(msg.to_lowercase().contains("unrealistic"));
    }

    #[test]
    fn test_validate_block_height_valid() {
        let (ok, _) = validate_block_height(700_000);
        assert!(ok);
    }

    #[test]
    fn test_halving_schedule() {
        let result = halving_schedule(&[0, 210_000, 420_000, 630_000]);
        assert_eq!(result.get(&0), Some(&5_000_000_000));
        assert_eq!(result.get(&210_000), Some(&2_500_000_000));
        assert_eq!(result.get(&420_000), Some(&1_250_000_000));
        assert_eq!(result.get(&630_000), Some(&625_000_000));
    }

    #[test]
    fn test_find_utxo_with_min_value() {
        let utxos = vec![
            Utxo {
                txid: "a".into(),
                vout: 0,
                value: 100,
            },
            Utxo {
                txid: "b".into(),
                vout: 0,
                value: 500,
            },
            Utxo {
                txid: "c".into(),
                vout: 0,
                value: 250,
            },
        ];
        let picked = find_utxo_with_min_value(&utxos, 200).unwrap();
        assert_eq!(picked.value, 250);
        assert_eq!(find_utxo_with_min_value(&utxos, 1000), None);
    }

    #[test]
    fn test_create_utxo() {
        let mut extra = HashMap::new();
        extra.insert("value".to_string(), "1000".to_string());
        extra.insert("script".to_string(), "OP_DUP".to_string());
        let result = create_utxo("abc123", 5, extra);
        assert_eq!(result.get("txid"), Some(&"abc123".to_string()));
        assert_eq!(result.get("vout"), Some(&"5".to_string()));
        assert_eq!(result.get("value"), Some(&"1000".to_string()));
        assert_eq!(result.get("script"), Some(&"OP_DUP".to_string()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_tx_version() {
        let tx_hex = "010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000";
        let version = extract_tx_version(tx_hex).unwrap();
        assert_eq!(version, 1);
    }

    #[test]
    fn test_version_2() {
        let tx_hex = "02000000000101706dc474338179f4ab8b7f0a4d07a2050113d7a0a9d21162e98b7319b102d3050100000000fdffffff02c9e10100000000001600148744bf9d300850a598b1a891f9a8d66524a4773065fc000000000000160014d1fae9a4de635c9c2e576238251d71be28a34dff0247304402201bf91432bbb345dcaa883a14fb7f18df7c821b160cc693f242112ba1a0acbdeb0220541b082c5fd4174f8eae782e213c1ebfc87b0598740ee0ef8463474debe83817012102062aea304064469ed250f46622e411de7eff4f07703e4273df6c80d58954ac2f00000000";
        let version = extract_tx_version(tx_hex).unwrap();
        assert_eq!(version, 2);
    }

    #[test]
    fn test_short_input() {
        let tx_hex = "00";
        let err = extract_tx_version(tx_hex).unwrap_err();
        assert!(err.contains("Transaction data too short"));
    }

    #[test]
    fn test_invalid_hex() {
        let tx_hex = "zzzzzzzz";
        let err = extract_tx_version(tx_hex).unwrap_err();
        assert!(err.contains("Hex decode error"));
    }
}
