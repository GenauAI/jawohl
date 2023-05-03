#[cfg(test)]
mod quickcheck_tests {
    use streaming_json_completer::complete_json::complete_json;

    #[test]
    fn test_property_based() {
        use quickcheck::{Arbitrary, Gen, QuickCheck};

        #[derive(Clone, Debug)]
        struct UntruncateTest {
            json: String,
        }
        impl Arbitrary for UntruncateTest {
            fn arbitrary(g: &mut Gen) -> Self {
                let json = String::arbitrary(g);
                UntruncateTest { json }
            }
        }


        fn prop(input: UntruncateTest) -> bool {
            let output = complete_json(&input.json);
            serde_json::from_str::<serde_json::Value>(&output).is_ok()
        }

        QuickCheck::new().tests(10_000).quickcheck(prop as fn(UntruncateTest) -> bool);
    }
}
