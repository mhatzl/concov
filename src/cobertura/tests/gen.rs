#[test]
fn gen_v4_read() {
    let content = include_str!("gen.xml");

    let cobertura = quick_xml::de::from_str::<crate::cobertura::schema::Coverage>(content).unwrap();

    insta::assert_ron_snapshot!(cobertura);
}

#[test]
fn gen_loose_read() {
    let content = include_str!("gen.xml");

    let cobertura =
        quick_xml::de::from_str::<crate::cobertura::schema_loose::Coverage>(content).unwrap();

    insta::assert_ron_snapshot!(cobertura);
}

#[test]
fn roundtrip() {
    let content = include_str!("gen.xml");
    let cobertura = quick_xml::de::from_str::<crate::cobertura::schema::Coverage>(content).unwrap();

    let ser_cobertura = cobertura.to_string();

    // removes whitespace formatting and possible x.0 for decimal representation
    let stripped_content = content
        .replace([' ', '\r', '\n', '\t'], "")
        .replace(".0\"", "\"");
    let stripped_cobertura = ser_cobertura
        .replace([' ', '\r', '\n', '\t'], "")
        .replace(".0\"", "\"");

    assert_eq!(
        stripped_cobertura, stripped_content,
        "Roundtrip cobertura doesn't macht original."
    );
}
