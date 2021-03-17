use linkme::distributed_slice;

#[distributed_slice]
pub static DOERS: [fn()] = [..];

pub fn dummy() {
    for doer in DOERS {
        doer();
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_dummy() {
        super::dummy();
    }
}
