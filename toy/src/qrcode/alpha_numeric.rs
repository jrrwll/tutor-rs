use std::collections::HashMap;
use std::lazy::SyncLazy;

static MAPPING: SyncLazy<HashMap<u8, u8>> = SyncLazy::new(|| {
    let mut m: HashMap<u8, u8> = HashMap::new();
    let cs = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:";
    let mut i = 0;
    for c in cs.chars() {
        m.insert(c as u8, i);
        i = i + 1;
    }
    m
});

fn map_char(c: &u8) -> u8 {
    MAPPING.get(c).map_or_else(|| 0, |v| *v)
}

pub fn encode_alpha_numeric(alpha_numeric_width: u8, chars: &str) -> Result<Vec<u8>, String> {
    let bytes = chars.as_bytes();
    let mut result = vec![];

    let mut prev: u8 = 0b0010;
    let mut expect = 4;

    let len = bytes.len();
    let mut i = 0;
    let mut size_append = false;
    while i < len {
        let mut remain = 11 - expect;
        let j = i + 2;
        let num: u16;
        if size_append {
            num = if j < len {
                (map_char(&bytes[i]) as u16) * 45 + (map_char(&bytes[i + 1]) as u16)
            } else {
                remain = 6 - expect;
                map_char(&bytes[i]) as u16
            };
            i = j;
        } else {
            num = len as u16;
            size_append = true;
            remain = alpha_numeric_width - expect;
        }

        let left = if prev != 0 { prev << expect } else { 0 };
        let right = (num >> remain) as u8;
        result.push(left | right);

        while remain >= 8 {
            let next_remain = remain - 8;
            result.push(((num >> next_remain) & 0xff) as u8);
            remain = next_remain;
        }
        prev = (num & ((1 << remain) - 1)) as u8;
        expect = 8 - remain;
        if i > len {
            result.push(prev);
            break
        }
    }
    Ok(result)
}
