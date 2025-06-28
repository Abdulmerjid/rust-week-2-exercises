use rust_week_2_exercises::*;

#[test]
fn test_decode_hex_and_endianness() {
    let hex_str = "deadbeef";
    let decoded = decode_hex(hex_str).unwrap();
    assert_eq!(decoded, vec![0xde, 0xad, 0xbe, 0xef]);

    let be = to_big_endian(&decoded);
    assert_eq!(be, vec![0xef, 0xbe, 0xad, 0xde]);
}

#[test]
fn test_hex_conversion() {
    let bytes = vec![0xde, 0xad, 0xbe, 0xef];
    let hex = bytes_to_hex(&bytes);
    assert_eq!(hex, "deadbeef");

    let back = hex_to_bytes(&hex).unwrap();
    assert_eq!(back, bytes);
}

#[test]
fn test_endianness_swap() {
    let num = 0x12345678;
    let le = swap_endian_u32(num);
    assert_eq!(le, [0x78, 0x56, 0x34, 0x12]);
}

#[test]
fn test_parse_satoshis_errors() {
    let bad_input = "notanumber";
    let result = parse_satoshis(bad_input);
    assert!(result.is_err());
}

#[test]
fn test_script_classification() {
    let p2pkh = vec![
        0x76, 0xa9, 0x14, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
        0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x88, 0xac,
    ];
    let p2wpkh = vec![
        0x00, 0x14, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
        0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13,
    ];
    let unknown = vec![0x01, 0x02, 0x03];

    assert!(matches!(classify_script(&p2pkh), ScriptType::P2PKH));
    assert!(matches!(classify_script(&p2wpkh), ScriptType::P2WPKH));
    assert!(matches!(classify_script(&unknown), ScriptType::Unknown));
}

#[test]
fn test_outpoint_destructuring() {
    let op = Outpoint(vec![1, 2, 3, 4], 42);
    let Outpoint(txid, vout) = op;
    assert_eq!(txid, vec![1, 2, 3, 4]);
    assert_eq!(vout, 42);
}

#[test]
fn test_script_slice() {
    let script = vec![0x6a, 0x24, 0xaa, 0xbb, 0xcc];
    let pushdata = read_pushdata(&script);
    assert_eq!(pushdata, &[0xaa, 0xbb, 0xcc]);
}

#[test]
fn test_wallet_balance_trait() {
    let wallet = TestWallet { confirmed: 5000 };
    assert_eq!(wallet.balance(), 5000);
}

#[test]
fn test_apply_fee() {
    let mut balance = 5000;
    apply_fee(&mut balance, 1000);
    assert_eq!(balance, 4000);

    apply_fee(&mut balance, 5000);
    assert_eq!(balance, 0);
}

#[test]
fn test_move_txid() {
    let result = move_txid("deadbeef".to_string());
    assert_eq!(result, "Moved txid: deadbeef");
}

#[test]
fn test_opcode_parsing() {
    assert_eq!(Opcode::from_byte(0xac), Ok(Opcode::OpChecksig));
    assert_eq!(Opcode::from_byte(0x76), Ok(Opcode::OpDup));
    assert_eq!(Opcode::from_byte(0x00), Ok(Opcode::OpInvalid));
}

#[test]
fn test_utxo_ownership() {
    let utxo = UTXO {
        txid: vec![0xde, 0xad, 0xbe, 0xef],
        vout: 0,
        value: 10000,
    };

    let result = consume_utxo(utxo.clone());
    assert_eq!(result, utxo);
}
