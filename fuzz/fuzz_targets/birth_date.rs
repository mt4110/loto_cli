#![no_main]

use libfuzzer_sys::fuzz_target;

#[path = "../../src/oracle_modules.rs"]
mod oracle;

fuzz_target!(|data: &[u8]| {
    let Some((&hour, input)) = data.split_first() else {
        return;
    };
    let Ok(input) = std::str::from_utf8(input) else {
        return;
    };
    let Ok(date) = oracle::parse_birth_date(input) else {
        return;
    };

    let pillars = oracle::pillars(date, Some(hour % 24));
    assert_eq!(pillars.len(), 4);
    assert_eq!(pillars[..3], oracle::pillars(date, None));
    for pillar in pillars {
        assert!(pillar.stem < 10);
        assert!(pillar.branch < 12);
        assert_eq!(pillar.to_string().chars().count(), 2);
    }
    assert!(oracle::western_sign(date).0 < 12);
    assert!(oracle::moon_phase(date).0 < 4);
    assert!((1..=9).contains(&oracle::numerology(date)));
});
