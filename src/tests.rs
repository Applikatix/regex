use super::*;

#[test]
fn regex_parse_test() {
    assert_eq!(
        Regex::new("abc").elements,
        vec![
            Elem { value: Value::Literal('a'), amount: Amount::One },
            Elem { value: Value::Literal('b'), amount: Amount::One },
            Elem { value: Value::Literal('c'), amount: Amount::One },
        ],
    );
    assert_eq!(
        Regex::new(r"a.\.").elements,
        vec![
            Elem { value: Value::Literal('a'), amount: Amount::One },
            Elem { value: Value::Wildcard, amount: Amount::One },
            Elem { value: Value::Literal('.'), amount: Amount::One },
        ],
    );
}

#[test]
fn match_elem_to_string() {
    let elements = Regex::new("abc").elements;

    assert!(elements[0].compare_value(&mut "a23".chars()));
    assert!(elements[1].compare_value(&mut "b".chars()));
    assert!(!elements[2].compare_value(&mut "a".chars()));
}

#[test]
fn regex_match_elems() {
    fn wrapper_eq(rx: &Regex, string: &str) {
        assert!(Elem::group_compare(&rx.elements, &mut string.chars()));
    }
    fn wrapper_ne(rx: &Regex, string: &str) {
        assert!(!Elem::group_compare(&rx.elements, &mut string.chars()));
    }

    let rx = Regex::new("abc");

    wrapper_eq(&rx, "abc");
    wrapper_eq(&rx, "abc456");
    wrapper_ne(&rx, "Abc");
    wrapper_ne(&rx, "abbc");
    wrapper_ne(&rx, "ab");
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
