use super::code::parse_bin;
use toy::qrcode::QrCode;

#[test]
fn test_encode_alpha_numeric() {
    // 0010 000000101 00111001110 11100111001 000010
    // 0010 0000|00101 001|11001110 | 11100111|001 00001|0
    // 00100000  32
    // 00101001  41
    // 11001110  206
    // 11100111  231
    // 00100001  33
    // 0         0
    let qr = QrCode::from_version(1);
    match qr.encode_alpha_numeric("AC-42") {
        Ok(v) => {
            let s: String = "0010 000000101 00111001110 11100111001 000010"
                .chars()
                .filter(|c| *c != ' ')
                .collect();
            let mut u = vec![];
            let mut i = vec![];
            for b in s.chars() {
                if i.len() != 8 {
                    i.push(b);
                    continue
                }
                u.push(parse_bin(i));
                i = vec![b];
            }
            u.push(parse_bin(i));

            println!("result: {:?}, expect: {:?}", &v, &u);
            assert_eq!(&v, &u);
        }
        Err(e) => {
            eprintln!("error: {}", e);
        }
    }
}
