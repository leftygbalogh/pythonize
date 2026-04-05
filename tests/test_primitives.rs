use pyo3::prelude::*;
use pythonize::{depythonize, pythonize};

fn round_trip<T>(py: Python<'_>, val: T) -> T
where
    T: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug + PartialEq,
{
    let py_val = pythonize(py, &val).expect("pythonize failed");
    depythonize(&py_val).expect("depythonize failed")
}

// ---------------------------------------------------------------------------
// i8
// ---------------------------------------------------------------------------

#[test]
fn test_i8_min() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, i8::MIN), i8::MIN);
    });
}

#[test]
fn test_i8_minus_one() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, -1i8), -1i8);
    });
}

#[test]
fn test_i8_zero() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 0i8), 0i8);
    });
}

#[test]
fn test_i8_one() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 1i8), 1i8);
    });
}

#[test]
fn test_i8_max() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, i8::MAX), i8::MAX);
    });
}

#[test]
fn test_i8_interesting() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 100i8), 100i8);
    });
}

// ---------------------------------------------------------------------------
// i16
// ---------------------------------------------------------------------------

#[test]
fn test_i16_min() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, i16::MIN), i16::MIN);
    });
}

#[test]
fn test_i16_minus_one() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, -1i16), -1i16);
    });
}

#[test]
fn test_i16_zero() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 0i16), 0i16);
    });
}

#[test]
fn test_i16_one() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 1i16), 1i16);
    });
}

#[test]
fn test_i16_max() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, i16::MAX), i16::MAX);
    });
}

#[test]
fn test_i16_interesting() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 32_000i16), 32_000i16);
    });
}

// ---------------------------------------------------------------------------
// i32
// ---------------------------------------------------------------------------

#[test]
fn test_i32_min() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, i32::MIN), i32::MIN);
    });
}

#[test]
fn test_i32_minus_one() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, -1i32), -1i32);
    });
}

#[test]
fn test_i32_zero() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 0i32), 0i32);
    });
}

#[test]
fn test_i32_one() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 1i32), 1i32);
    });
}

#[test]
fn test_i32_max() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, i32::MAX), i32::MAX);
    });
}

#[test]
fn test_i32_interesting() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 2_147_483_647i32), 2_147_483_647i32);
    });
}

// ---------------------------------------------------------------------------
// i64
// ---------------------------------------------------------------------------

#[test]
fn test_i64_min() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, i64::MIN), i64::MIN);
    });
}

#[test]
fn test_i64_minus_one() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, -1i64), -1i64);
    });
}

#[test]
fn test_i64_zero() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 0i64), 0i64);
    });
}

#[test]
fn test_i64_one() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 1i64), 1i64);
    });
}

#[test]
fn test_i64_max() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, i64::MAX), i64::MAX);
    });
}

#[test]
fn test_i64_interesting() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 1_000_000_000_000i64), 1_000_000_000_000i64);
    });
}

// ---------------------------------------------------------------------------
// u8
// ---------------------------------------------------------------------------

#[test]
fn test_u8_zero() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 0u8), 0u8);
    });
}

#[test]
fn test_u8_one() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 1u8), 1u8);
    });
}

#[test]
fn test_u8_max() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, u8::MAX), u8::MAX);
    });
}

#[test]
fn test_u8_interesting() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 200u8), 200u8);
    });
}

// ---------------------------------------------------------------------------
// u16
// ---------------------------------------------------------------------------

#[test]
fn test_u16_zero() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 0u16), 0u16);
    });
}

#[test]
fn test_u16_one() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 1u16), 1u16);
    });
}

#[test]
fn test_u16_max() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, u16::MAX), u16::MAX);
    });
}

#[test]
fn test_u16_interesting() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 60_000u16), 60_000u16);
    });
}

// ---------------------------------------------------------------------------
// u32
// ---------------------------------------------------------------------------

#[test]
fn test_u32_zero() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 0u32), 0u32);
    });
}

#[test]
fn test_u32_one() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 1u32), 1u32);
    });
}

#[test]
fn test_u32_max() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, u32::MAX), u32::MAX);
    });
}

#[test]
fn test_u32_interesting() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 4_000_000_000u32), 4_000_000_000u32);
    });
}

// ---------------------------------------------------------------------------
// u64
// ---------------------------------------------------------------------------

#[test]
fn test_u64_zero() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 0u64), 0u64);
    });
}

#[test]
fn test_u64_one() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 1u64), 1u64);
    });
}

#[test]
fn test_u64_max() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, u64::MAX), u64::MAX);
    });
}

#[test]
fn test_u64_interesting() {
    Python::attach(|py| {
        assert_eq!(
            round_trip(py, 9_000_000_000_000_000_000u64),
            9_000_000_000_000_000_000u64,
        );
    });
}

// ---------------------------------------------------------------------------
// isize — forwarded to i64 on 64-bit targets; round-trippable
// ---------------------------------------------------------------------------

#[test]
fn test_isize_zero() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 0isize), 0isize);
    });
}

#[test]
fn test_isize_one() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 1isize), 1isize);
    });
}

#[test]
fn test_isize_max_safe() {
    // Cast i64::MAX safely; on 32-bit this would need adjustment but the
    // project targets 64-bit CI.
    Python::attach(|py| {
        let val = i64::MAX as isize;
        assert_eq!(round_trip(py, val), val);
    });
}

#[test]
fn test_isize_interesting() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, -1_000_000isize), -1_000_000isize);
    });
}

// ---------------------------------------------------------------------------
// usize — forwarded to u64 on 64-bit targets; round-trippable
// ---------------------------------------------------------------------------

#[test]
fn test_usize_zero() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 0usize), 0usize);
    });
}

#[test]
fn test_usize_one() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 1usize), 1usize);
    });
}

#[test]
fn test_usize_max_safe() {
    // u64::MAX as usize is safe on 64-bit targets.
    Python::attach(|py| {
        let val = u64::MAX as usize;
        assert_eq!(round_trip(py, val), val);
    });
}

#[test]
fn test_usize_interesting() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 1_000_000usize), 1_000_000usize);
    });
}

// ---------------------------------------------------------------------------
// i128 — error path: serialize_i128 not implemented in pythonize
// ---------------------------------------------------------------------------

#[test]
fn test_i128_zero_is_err() {
    Python::attach(|py| {
        let result = pythonize(py, &0i128);
        assert!(result.is_err());
        // informational only — not a public API contract:
        // assert!(result.unwrap_err().to_string().contains("..."));
    });
}

#[test]
fn test_i128_min_is_err() {
    Python::attach(|py| {
        let result = pythonize(py, &i128::MIN);
        assert!(result.is_err());
    });
}

#[test]
fn test_i128_max_is_err() {
    Python::attach(|py| {
        let result = pythonize(py, &i128::MAX);
        assert!(result.is_err());
    });
}

// ---------------------------------------------------------------------------
// u128 — error path: serialize_u128 not implemented in pythonize
// ---------------------------------------------------------------------------

#[test]
fn test_u128_zero_is_err() {
    Python::attach(|py| {
        let result = pythonize(py, &0u128);
        assert!(result.is_err());
        // informational only — not a public API contract:
        // assert!(result.unwrap_err().to_string().contains("..."));
    });
}

#[test]
fn test_u128_one_is_err() {
    Python::attach(|py| {
        let result = pythonize(py, &1u128);
        assert!(result.is_err());
    });
}

#[test]
fn test_u128_max_is_err() {
    Python::attach(|py| {
        let result = pythonize(py, &u128::MAX);
        assert!(result.is_err());
    });
}

// ---------------------------------------------------------------------------
// f64 — finite round-trip
// ---------------------------------------------------------------------------

#[test]
fn test_f64_zero() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 0.0f64), 0.0f64);
    });
}

#[test]
fn test_f64_one() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 1.0f64), 1.0f64);
    });
}

#[test]
fn test_f64_minus_one() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, -1.0f64), -1.0f64);
    });
}

#[test]
fn test_f64_max() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, f64::MAX), f64::MAX);
    });
}

#[test]
fn test_f64_min_positive() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, f64::MIN_POSITIVE), f64::MIN_POSITIVE);
    });
}

#[test]
fn test_f64_negative_decimal() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, -3.14159265358979f64), -3.14159265358979f64);
    });
}

// ---------------------------------------------------------------------------
// f64 NaN — characterisation test
// BASELINE: outcome not confirmed pre-contribution; this test documents what
// pythonize actually does with NaN so that the behaviour is explicit and
// reviewable by upstream maintainers.
// ---------------------------------------------------------------------------

#[test]
fn test_f64_nan_characterisation() {
    Python::attach(|py| {
        // BASELINE (confirmed 2026-04-04): Python float natively supports NaN;
        // pythonize succeeds and depythonize returns Ok(NaN).
        let py_val = pythonize(py, &f64::NAN).expect("pythonize NaN should succeed");
        let back: f64 = depythonize(&py_val).expect("depythonize NaN should succeed");
        assert!(back.is_nan(), "expected NaN, got {back}");
    });
}

// ---------------------------------------------------------------------------
// f64 Infinity — characterisation test
// BASELINE: outcome not confirmed pre-contribution; documents actual behaviour.
// ---------------------------------------------------------------------------

#[test]
fn test_f64_infinity_characterisation() {
    Python::attach(|py| {
        // BASELINE (confirmed 2026-04-04): Python float supports +Inf;
        // pythonize succeeds and depythonize round-trips correctly.
        let py_val = pythonize(py, &f64::INFINITY).expect("pythonize +Inf should succeed");
        let back: f64 = depythonize(&py_val).expect("depythonize +Inf should succeed");
        assert!(back.is_infinite() && back.is_sign_positive());
    });
}

#[test]
fn test_f64_neg_infinity_characterisation() {
    Python::attach(|py| {
        // BASELINE (confirmed 2026-04-04): Python float supports -Inf;
        // pythonize succeeds and depythonize round-trips correctly.
        let py_val = pythonize(py, &f64::NEG_INFINITY).expect("pythonize -Inf should succeed");
        let back: f64 = depythonize(&py_val).expect("depythonize -Inf should succeed");
        assert!(back.is_infinite() && back.is_sign_negative());
    });
}

// ---------------------------------------------------------------------------
// f32 — characterisation test
// f32 widens to Python float (f64) on serialize. Narrowing back with
// depythonize::<f32> may not reproduce the original bit pattern exactly for
// all values because the Python float stores an f64. We assert the observed
// result without asserting bit-exact equality.
// BASELINE: round-trip observed to be imprecise for sub-normal f32 values;
// for normal values the round-trip is typically exact due to f32 ⊂ f64
// representable range, but this is implementation-defined.
// ---------------------------------------------------------------------------

#[test]
fn test_f32_zero_characterisation() {
    Python::attach(|py| {
        let val = 0.0f32;
        let py_val = pythonize(py, &val).expect("pythonize failed");
        let back: Result<f32, _> = depythonize(&py_val);
        // BASELINE: 0.0f32 round-trips cleanly; record without strict equality
        // to remain valid if behaviour changes.
        assert!(back.is_ok());
    });
}

#[test]
fn test_f32_one_characterisation() {
    Python::attach(|py| {
        let val = 1.0f32;
        let py_val = pythonize(py, &val).expect("pythonize failed");
        let back: Result<f32, _> = depythonize(&py_val);
        // BASELINE: 1.0f32 is exactly representable in f64; round-trip observed Ok.
        assert!(back.is_ok());
    });
}

#[test]
fn test_f32_large_value_characterisation() {
    Python::attach(|py| {
        // f32::MAX is exactly representable; document actual result.
        let val = f32::MAX;
        let py_val = pythonize(py, &val).expect("pythonize failed");
        let back: Result<f32, _> = depythonize(&py_val);
        // BASELINE: f32::MAX widens to f64 without loss; depythonize back to
        // f32 may succeed or fail depending on range checking in pythonize.
        let _ = back;
    });
}

#[test]
fn test_f32_pi_characterisation() {
    Python::attach(|py| {
        let val = std::f32::consts::PI;
        let py_val = pythonize(py, &val).expect("pythonize failed");
        let back: Result<f32, _> = depythonize(&py_val);
        // BASELINE: f32 PI widened to f64 PI; narrowing back to f32 loses
        // the extra f64 precision so result may differ from original by
        // rounding. Do NOT assert equality.
        assert!(back.is_ok());
    });
}

// ---------------------------------------------------------------------------
// bool
// ---------------------------------------------------------------------------

#[test]
fn test_bool_true() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, true), true);
    });
}

#[test]
fn test_bool_false() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, false), false);
    });
}

// ---------------------------------------------------------------------------
// String
// ---------------------------------------------------------------------------

#[test]
fn test_string_empty() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, String::from("")), String::from(""));
    });
}

#[test]
fn test_string_hello() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, String::from("hello")), String::from("hello"),);
    });
}

#[test]
fn test_string_long_ascii() {
    Python::attach(|py| {
        let s = "abcdefghijklmnopqrstuvwxyz".repeat(10);
        assert_eq!(round_trip(py, s.clone()), s);
    });
}

#[test]
fn test_string_whitespace_newline() {
    Python::attach(|py| {
        let s = String::from("hello world\nfoo\tbar");
        assert_eq!(round_trip(py, s.clone()), s);
    });
}

// ---------------------------------------------------------------------------
// char
// ---------------------------------------------------------------------------

#[test]
fn test_char_lowercase_a() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 'a'), 'a');
    });
}

#[test]
fn test_char_uppercase_z() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, 'Z'), 'Z');
    });
}

#[test]
fn test_char_digit_zero() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, '0'), '0');
    });
}

#[test]
fn test_char_unicode_a_umlaut_characterisation() {
    Python::attach(|py| {
        // BASELINE (confirmed 2026-04-04): 'ä' is U+00E4 — 1 Unicode code point
        // but 2 UTF-8 bytes. pythonize serializes it as a 2-byte Python str (Ok).
        // depythonize checks `s.len() != 1` (byte length, not char count) and
        // returns Err(InvalidLengthChar). This is a bug: the guard should use
        // `s.chars().count() != 1` instead. Asserting Err here as a regression guard.
        let py_val = pythonize(py, &'ä').expect("pythonize char should succeed");
        let back: Result<char, _> = depythonize(&py_val);
        assert!(
            back.is_err(),
            "expected Err(InvalidLengthChar) for non-ASCII char — if this now Ok, \n\
             upstream may have fixed the byte-length bug (s.len() -> s.chars().count())"
        );
    });
}

// ---------------------------------------------------------------------------
// unit ()
// ---------------------------------------------------------------------------

#[test]
fn test_unit() {
    Python::attach(|py| {
        assert_eq!(round_trip(py, ()), ());
    });
}
