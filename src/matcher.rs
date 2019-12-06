use std::iter::Peekable;

pub fn do_match<'a, I>(iter: &mut Peekable<I>) -> Vec<u8>
    where I: Iterator<Item = &'a u8>
    // FIXME does this need to be peekable?
{
    let mut buffer: Vec<u8> = Vec::new(); // FIXME capacity?

    loop {
        match iter.peek() {
            Some(b'a') => {
                iter.next();
                match iter.peek() {
                    Some(b'a') => {
                        iter.next();
                        match iter.peek() {
                            Some(b'a') => {
                                iter.next();
                                buffer.extend_from_slice(&[b'A']);
                            },
                            _ => {
                                buffer.extend_from_slice(&[b'a', b'a']);
                            },
                        }
                    },
                    _ => {
                        buffer.extend_from_slice(&[b'a']);
                    },
                }
            },
            Some(b'b') => {
                iter.next();
                match iter.peek() {
                    Some(b'b') => {
                        iter.next();
                        buffer.extend_from_slice(&[b'B']);
                    }
                    _ => {
                        buffer.extend_from_slice(&[b'b']);
                    },
                }
            }
            Some(_) => buffer.push(*iter.next().expect("peek and next didn't match")),
            None => return buffer,
        }
    }
}

pub fn m<S: AsRef<[u8]>>(escaped: S) -> String {
    let escaped = escaped.as_ref();
    let mut iter = escaped.iter().peekable();
    let buffer = do_match(&mut iter);
    if let Some(_) = iter.next() {
        unreachable!("this should be a no-op");
    }
    String::from_utf8(buffer).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    test!(matcher_abc, m("abc") == "abc");
    test!(matcher_aab, m("aab") == "aab");
    test!(matcher_aaa, m("aaa") == "A");
    test!(matcher_abaaa, m("abaaa") == "abA");
    test!(matcher_aaaa, m("aaaa") == "Aa");
    test!(matcher_baaa, m("baaa") == "bA");
    test!(matcher_bcaaa, m("bcaaa") == "bcA");
    test!(matcher_bcaaaa, m("bcaaaa") == "bcAa");
    test!(matcher_bcaaaab, m("bcaaaab") == "bcAab");
    test!(matcher_baaaaaab, m("baaaaaab") == "bAAb");
    test!(matcher_baaasaaab, m("baaasaaab") == "bAsAb");
    test!(matcher_abbc, m("abbc") == "aBc");
    test!(matcher_aabb, m("aabb") == "aaB");
}
