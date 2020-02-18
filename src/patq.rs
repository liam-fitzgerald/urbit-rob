use std::convert::TryInto;

use super::phonemes::{PRE,SUF};

pub fn bytes_to_patq(buf: Vec<u8>) -> String {

    let mut result = String::from("~");

    for i in (0..buf.len()).rev() {
        let syl = if i % 2 == 0 {
            SUF[buf[i] as usize]
        } else {
            PRE[buf[i] as usize]
        };
        result.push_str(syl);
        if i % 2 == 0 && i != 0 {
            result.push_str("-");
        }
    }
    result
}

pub fn patq_to_bytes(patq: &mut String) -> Option<Vec<u8>> {
    let mut result: Vec<u8> = Vec::new();


    if patq.remove(0) != '~' {
        return None
    }

    let suffixes = SUF.to_vec().into_iter();
    let prefixes = PRE.to_vec().into_iter();

    let syls = patq.split('-').map(|s| { s.split_at(3) }).collect::<Vec<_>>();

    for i in (0..syls.len()).rev() {
        let (prefix, suffix) = syls[i];
        if suffix.len() == 0 {
            let byte = suffixes.clone().position(|s| { s == prefix })?;
            result.push(byte.try_into().unwrap());
        } else {
            let first = suffixes.clone().position(|s| { s == suffix })?;
            let second = prefixes.clone().position(|p| { p == prefix })?;
            result.push(first.try_into().unwrap());
            result.push(second.try_into().unwrap());
        }
    }

    Some(result)
}
