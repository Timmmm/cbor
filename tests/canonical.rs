#[cfg(feature = "std")]
mod std_tests {
    use serde_cbor::value::Value;

    #[test]
    fn integer_canonical_sort_order() {
        let expected = [
            0,
            23,
            24,
            255,
            256,
            65535,
            65536,
            4294967295,
            -1,
            -24,
            -25,
            -256,
            -257,
            -65536,
            -65537,
            -4294967296,
        ]
        .into_iter()
        .map(|i| Value::I64(*i))
        .collect::<Vec<_>>();

        let mut sorted = expected.clone();
        sorted.sort();

        assert_eq!(expected, sorted);
    }

    #[test]
    fn string_canonical_sort_order() {
        let expected = ["", "a", "b", "aa"]
            .into_iter()
            .map(|s| Value::String(s.to_string()))
            .collect::<Vec<_>>();

        let mut sorted = expected.clone();
        sorted.sort();

        assert_eq!(expected, sorted);
    }

    #[test]
    fn bytes_canonical_sort_order() {
        let expected = vec![vec![], vec![0u8], vec![1u8], vec![0u8, 0u8]]
            .into_iter()
            .map(|v| Value::Bytes(v))
            .collect::<Vec<_>>();

        let mut sorted = expected.clone();
        sorted.sort();

        assert_eq!(expected, sorted);
    }

    #[test]
    fn simple_data_canonical_sort_order() {
        let expected = vec![Value::Bool(false), Value::Bool(true), Value::Null];

        let mut sorted = expected.clone();
        sorted.sort();

        assert_eq!(expected, sorted);
    }

    #[test]
    fn major_type_canonical_sort_order() {
        let expected = vec![
            Value::I64(0),
            Value::I64(-1),
            Value::Bytes(vec![]),
            Value::String("".to_string()),
            Value::Null,
        ];

        let mut sorted = expected.clone();
        sorted.sort();

        assert_eq!(expected, sorted);
    }

    #[test]
    fn test_rfc_example() {
        // See: https://tools.ietf.org/html/draft-ietf-cbor-7049bis-04#section-4.10
        let expected = vec![
            Value::I64(10),
            Value::I64(100),
            Value::I64(-1),
            Value::String("z".to_owned()),
            Value::String("aa".to_owned()),
            Value::Array(vec![Value::I64(100)]),
            Value::Array(vec![Value::I64(-1)]),
            Value::Bool(false),
        ];
        let mut sorted = expected.clone();
        sorted.sort();
        assert_eq!(expected, sorted);
    }
}
