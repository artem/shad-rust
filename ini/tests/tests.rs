use ini::{parse, IniFile};

use pretty_assertions::assert_eq;

#[test]
fn test_simple() {
    let ini = parse(
        "[section]\n\
         key=value",
    );

    let mut expected = IniFile::new();
    expected.insert(
        "section".to_string(),
        vec![("key".to_string(), "value".to_string())]
            .into_iter()
            .collect(),
    );

    assert_eq!(ini, expected);
}

#[test]
fn test_whitespaces() {
    let ini =
        parse(" \n  [  section\t]\n   \tkey lolo  hohoho \t=\r   value   after  spaces  \t\n");

    let mut expected = IniFile::new();
    expected.insert(
        "  section\t".to_string(),
        vec![(
            "key lolo  hohoho".to_string(),
            "value   after  spaces".to_string(),
        )]
        .into_iter()
        .collect(),
    );

    assert_eq!(ini, expected);
}

#[test]
fn test_complex() {
    let ini = parse(
        "  \t[foo_section]\n\
         a=b\n\
         \t   \n\
         \n\
         foo = bar\n\
         [bar_section]  \n\
         hello,\tworld    =  hey   bro\n\
         \n\
         [   \tbaz\t section ]  \n\
         key   =    value\n\
         \t\n\
         \n",
    );

    let mut expected = IniFile::new();
    expected.insert(
        "foo_section".to_string(),
        vec![
            ("a".to_string(), "b".to_string()),
            ("foo".to_string(), "bar".to_string()),
        ]
        .into_iter()
        .collect(),
    );
    expected.insert(
        "bar_section".to_string(),
        vec![("hello,\tworld".to_string(), "hey   bro".to_string())]
            .into_iter()
            .collect(),
    );
    expected.insert(
        "   \tbaz\t section ".to_string(),
        vec![("key".to_string(), "value".to_string())]
            .into_iter()
            .collect(),
    );

    assert_eq!(ini, expected);
}

#[test]
fn test_sections_union() {
    let ini = parse(
        "[section]\n\
         key=value\n\
         [section]\n\
         foo=bar",
    );

    let mut expected = IniFile::new();
    expected.insert(
        "section".to_string(),
        vec![
            ("key".to_string(), "value".to_string()),
            ("foo".to_string(), "bar".to_string()),
        ]
        .into_iter()
        .collect(),
    );

    assert_eq!(ini, expected);
}

#[test]
fn test_sections_overwrite() {
    let ini = parse(
        "[section]\n\
         key=value\n\
         [section]\n\
         key=bar",
    );

    let mut expected = IniFile::new();
    expected.insert(
        "section".to_string(),
        vec![("key".to_string(), "bar".to_string())]
            .into_iter()
            .collect(),
    );

    assert_eq!(ini, expected);
}

#[test]
fn test_empty() {
    assert_eq!(parse(""), IniFile::new());
    assert_eq!(parse("   "), IniFile::new());
    assert_eq!(parse("  \n\n\t\n\t \t   \n"), IniFile::new());
}

#[test]
fn test_empty_section() {
    let ini = parse("[section]");

    let mut expected = IniFile::new();
    expected.entry("section".to_string()).or_default();

    assert_eq!(ini, expected);
}

#[test]
fn test_empty_value() {
    const FILES: &[&str] = &[
        "[section]\n\
         abra =     \n\
         cadabra=\n\
         chpok",
        "[section]\n\
         abra =     \n\
         cadabra\n\
         chpok=",
        "[section]\n\
         abra   \n\
         cadabra=\n\
         chpok =    \n\t ",
        "[section]\n\
         abra\n\
         cadabra=  \n\
         chpok      \n\t ",
    ];

    let mut expected = IniFile::new();
    expected.insert(
        "section".to_string(),
        vec![
            ("abra".to_string(), "".to_string()),
            ("cadabra".to_string(), "".to_string()),
            ("chpok".to_string(), "".to_string()),
        ]
        .into_iter()
        .collect(),
    );

    for file in FILES {
        eprintln!("Testing case:\n{}", file);
        assert_eq!(parse(file), expected);
    }
}

#[test]
fn test_utf8() {
    let ini = parse(
        "[Cекция]\n\
         АА1=отлично\n\
         АА2 = нормально, но можно лучше\n\
         АА3
         [部分]\n\
         Schlüssel = lang værdi\n\
         מַפְתֵחַ =
         مفتاح",
    );

    let mut expected = IniFile::new();
    expected.insert(
        "Cекция".to_string(),
        vec![
            ("АА1".to_string(), "отлично".to_string()),
            ("АА2".to_string(), "нормально, но можно лучше".to_string()),
            ("АА3".to_string(), "".to_string()),
        ]
        .into_iter()
        .collect(),
    );
    expected.insert(
        "部分".to_string(),
        vec![
            ("Schlüssel".to_string(), "lang værdi".to_string()),
            ("מַפְתֵחַ".to_string(), "".to_string()),
            ("مفتاح".to_string(), "".to_string()),
        ]
        .into_iter()
        .collect(),
    );

    assert_eq!(ini, expected);
}

#[test]
#[should_panic]
fn test_stray_pair() {
    parse("hello = world");
}

#[test]
#[should_panic]
fn test_stray_key() {
    parse("hello =");
}

#[test]
#[should_panic]
fn test_missing_bracket() {
    parse(
        "[section\n\
         abra = cadabra",
    );
}

#[test]
#[should_panic]
fn test_double_bracket() {
    parse(
        "[[section]]\n\
         abra = cadabra",
    );
}

#[test]
#[should_panic]
fn test_triple_equals() {
    parse(
        "[section]\n\
         abra = cadabra=foo",
    );
}
