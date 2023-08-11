use super::*;

#[test]
fn regex_parse_test() {
    assert_eq!(
        Regex::new("abc").elements,
        vec![
            Elem { value: Value::Literal('a'), quantity: Amount::One },
            Elem { value: Value::Literal('b'), quantity: Amount::One },
            Elem { value: Value::Literal('c'), quantity: Amount::One },
        ],
    );
}

#[test]
fn regex_match_elems() {
    let rx = Regex::new("abc");

    assert!(rx.match_elems("abc".chars()));
    assert!(rx.match_elems("abc456".chars()));
    assert!(!rx.match_elems("Abc".chars()));
    assert!(!rx.match_elems("abbc".chars()));
    assert!(!rx.match_elems("ab".chars()));
}

#[test]
fn regex_match_literals() {
    let rx = Regex::new("abc");

    assert!(rx.is_match("abc"));
    assert!(rx.is_match("123abc456"));
    assert!(!rx.is_match("Abc"));
    assert!(!rx.is_match("abbc"));
    assert!(!rx.is_match("ab"));
}
