use hex::decode;
use rust_week_2_exercises::*;

#[test]
fn test_decode_hex_and_endianness() {
    let hex_str = "deadbeef";
    let decoded = decode_hex(hex_str).unwrap();
    let big_endian = to_big_endian(&decoded);
    assert_eq!(big_endian, vec![0xef, 0xbe, 0xad, 0xde]);
}

#[test]
fn test_hex_conversion() {
    let bytes = vec![0xde, 0xad, 0xbe, 0xef];
    let hex = bytes_to_hex(&bytes);
    let back = hex_to_bytes(&hex).unwrap();
    assert_eq!(bytes, back);
}

#[test]
fn test_endianness_swap() {
    let num = 0x12345678;
    let le = swap_endian_u32(num);
    assert_eq!(le, [0x78, 0x56, 0x34, 0x12]);
}

#[test]
fn test_parse_satoshis_errors() {
    let input = "not_a_number";
    let result = parse_satoshis(input);
    assert!(result.is_err());
}

#[test]
fn test_script_classification() {
    let p2pkh = vec![
        0x76, 0xa9, 0x14, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c,
        0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x88, 0xac,
    ];
    let p2wpkh = vec![
        0x00, 0x14, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
        0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13,
    ];
    let unknown = vec![0x01, 0x02, 0x03];

    assert!(matches!(classify_script(&p2pkh), ScriptType::P2PKH));
    assert!(matches!(classify_script(&p2wpkh), ScriptType::P2WPKH));
    assert!(matches!(classify_script(&unknown), ScriptType::Unknown));
}

#[test]
fn test_script_slice() {
    let script = vec![0x6a, 0x24, 0xaa, 0xbb, 0xcc];
    assert_eq!(read_pushdata(&script), &[0xaa, 0xbb, 0xcc]);
}

#[test]
fn test_outpoint_destructuring() {
    let outpoint = Outpoint(vec![1, 2, 3], 42);
    let Outpoint(txid, vout) = outpoint;
    assert_eq!(txid, vec![1, 2, 3]);
    assert_eq!(vout, 42);
}

#[test]
fn test_wallet_balance_trait() {
    let wallet = TestWallet { confirmed: 500 };
    assert_eq!(wallet.balance(), 500);
}

#[test]
fn test_apply_fee() {
    let mut balance = 100;
    apply_fee(&mut balance, 30);
    assert_eq!(balance, 70);

    apply_fee(&mut balance, 100);
    assert_eq!(balance, 0);
}

#[test]
fn test_move_txid() {
    let txid = "deadbeef".to_string();
    let result = move_txid(txid.clone());
    assert_eq!(result, format!("txid: {}", txid));
}

#[test]
fn test_opcode_parsing() {
    assert_eq!(Opcode::from_byte(0xac), Ok(Opcode::OP_CHECKSIG));
    assert_eq!(Opcode::from_byte(0x76), Ok(Opcode::OP_DUP));
    assert_eq!(
        Opcode::from_byte(0x00),
        Err("Invalid opcode: 0x00".to_string())
    );
}

#[test]
fn test_utxo_ownership() {
    let utxo = UTXO {
        txid: vec![0xaa, 0xbb],
        vout: 1,
        value: 1000,
    };
    let consumed = consume_utxo(utxo.clone());
    assert_eq!(utxo, consumed);
}
