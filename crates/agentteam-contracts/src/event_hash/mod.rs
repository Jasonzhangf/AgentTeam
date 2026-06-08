pub fn event_payload_hash(payload: &str) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in payload.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("fnv1a64-{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_payload_hash_is_stable() {
        assert_eq!(
            event_payload_hash("{\"event\":\"task_created\"}"),
            "fnv1a64-a6194b4867e72d61"
        );
    }
}
