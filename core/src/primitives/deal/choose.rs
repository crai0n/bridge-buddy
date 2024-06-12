pub fn choose(n: u8, k: u8) -> u128 {
    // taken from https://blog.plover.com/math/choose.html
    if k > n {
        0
    } else {
        let mut r = 1;
        let mut n = n as u128;

        for d in 1..=k {
            r *= n;
            r /= d as u128; // this will always be integer
            n -= 1;
        }
        r
    }
}

pub fn sequence_to_index(seq: [u8; 13]) -> u128 {
    seq.iter().enumerate().fold(0, |_, (i, x)| choose(*x, i as u8))
}

pub fn index_to_sequence(index: u128) -> [u8; 13] {
    let mut result = [0; 13];
    let mut current_index = index;
    for l in (1..=13u8).rev() {
        // the lth element of the sequence is k such that
        // k choose l < current_index and (k+1) choose l >= current_index
        // while current_index
        let mut k = l;
        let mut prev_value = 0;
        let mut next_value = 1;
        while next_value < current_index {
            k += 1;
            prev_value = next_value;
            next_value = choose(k, l);
        }
        current_index -= prev_value;
        result[(l - 1) as usize] = k - 1;
    }
    result
}
