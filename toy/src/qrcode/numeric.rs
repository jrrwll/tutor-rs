pub fn encode_numeric(numeric_width: u8, nums: &str) -> Result<Vec<u8>, String> {
    let mut result = vec![];

    let mut prev: u8 = 0b0001;
    let mut expect = 4;

    let len = nums.len();
    let mut i = 0;
    let mut size_append = false;
    while i < len {
        let mut width = numeric_width;
        let mut remain = width - expect;
        let mut j = i + 3;
        let diff = j as i64 - len as i64;
        if diff > 0 {
            j = len;
            if diff == 1 {
                width = 7;
            } else if diff == 2 {
                width = 4;
            }
            if width > expect {
                remain = width - expect;
            } else {
                remain = 0;
            }
        }
        let num: u16;
        if size_append {
            num = nums[i..j].parse::<u16>().map_err(|e| e.to_string())?;
            i = j;
        } else {
            num = len as u16;
            size_append = true;
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
        if i == len {
            result.push(prev);
            break
        }
    }
    Ok(result)
}
