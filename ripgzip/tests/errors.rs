fn check_decompression_error(mut data: &[u8], msg: &'static str) {
    let res = ripgzip::decompress(&mut data, &mut std::io::sink());
    if res.is_ok() {
        panic!("expected Err, got Ok");
    }
    for inner in res.unwrap_err().chain() {
        if inner.to_string().contains(msg) {
            return;
        }
    }
    panic!("error does not contain message: {}", msg);
}

#[test]
fn errors() {
    check_decompression_error(
        include_bytes!("../data/corrupted/00-bad-length.gz"),
        "length check failed",
    );
    check_decompression_error(
        include_bytes!("../data/corrupted/01-bad-crc32.gz"),
        "crc32 check failed",
    );
    check_decompression_error(include_bytes!("../data/corrupted/02-unexpected-eof.gz"), "");
    check_decompression_error(
        include_bytes!("../data/corrupted/03-wrong-id.gz"),
        "wrong id values",
    );
    check_decompression_error(include_bytes!("../data/corrupted/04-header-eof.gz"), "");
    check_decompression_error(
        include_bytes!("../data/corrupted/05-bad-header-crc16.gz"),
        "header crc16 check failed",
    );
    check_decompression_error(
        include_bytes!("../data/corrupted/06-invalid-btype.gz"),
        "unsupported block type",
    );
    check_decompression_error(
        include_bytes!("../data/corrupted/07-invalid-cm.gz"),
        "unsupported compression method",
    );
    check_decompression_error(
        include_bytes!("../data/corrupted/08-bad-nlen.gz"),
        "nlen check failed",
    );
}
