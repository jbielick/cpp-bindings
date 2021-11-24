#[allow(unused_imports)]
use crate::{blob_type, bytes::ByteListBlob, string::StringBlob};
use lib_ruby_parser::source::{DecoderResult, InputError};

blob_type!(InputErrorBlob, InputError);
blob_type!(DecoderResultBlob, DecoderResult);
blob_type!(MaybeDecoderBlob, Option<Decoder>);

#[cfg(feature = "tests")]
#[no_mangle]
pub extern "C" fn lib_ruby_parser__test__make_input_error__unsupported_encoding(
    s: StringBlob,
) -> InputErrorBlob {
    let s = String::from(s);
    InputErrorBlob::from(InputError::UnsupportedEncoding(s))
}

#[cfg(feature = "tests")]
#[no_mangle]
pub extern "C" fn lib_ruby_parser__test__make_input_error__decoding_error(
    s: StringBlob,
) -> InputErrorBlob {
    InputErrorBlob::from(InputError::DecodingError(String::from(s)))
}

#[no_mangle]
pub extern "C" fn LIB_RUBY_PARSER_drop_input_error(input_error: *mut InputError) {
    unsafe { std::ptr::drop_in_place(input_error) }
}

#[cfg(feature = "tests")]
#[no_mangle]
pub extern "C" fn lib_ruby_parser__test__make_decoder_result__ok(
    bytes: ByteListBlob,
) -> DecoderResultBlob {
    DecoderResultBlob::from(DecoderResult::Ok(bytes.into()))
}

#[cfg(feature = "tests")]
#[no_mangle]
pub extern "C" fn lib_ruby_parser__test__make_decoder_result__err(
    err: InputErrorBlob,
) -> DecoderResultBlob {
    DecoderResultBlob::from(DecoderResult::Err(err.into()))
}

#[no_mangle]
pub extern "C" fn LIB_RUBY_PARSER_drop_decoder_result(decoder_result: *mut DecoderResult) {
    unsafe { std::ptr::drop_in_place(decoder_result) }
}

#[repr(C)]
#[derive(Debug)]
pub struct Decoder {
    pub f: extern "C" fn(encoding: StringBlob, input: ByteListBlob) -> DecoderResultBlob,
}

#[cfg(feature = "tests")]
#[no_mangle]
pub extern "C" fn lib_ruby_parser__test__always_ok_decoder() -> Decoder {
    #[no_mangle]
    pub extern "C" fn lib_ruby_parser__test__always_ok_decoder_fn(
        encoding: StringBlob,
        input: ByteListBlob,
    ) -> DecoderResultBlob {
        // do cleanup
        drop(String::from(encoding));
        drop(Vec::<u8>::from(input));
        // and return constant output
        DecoderResultBlob::from(DecoderResult::Ok("always_ok".as_bytes().to_vec()))
    }
    Decoder {
        f: lib_ruby_parser__test__always_ok_decoder_fn,
    }
}

#[cfg(feature = "tests")]
#[no_mangle]
pub extern "C" fn lib_ruby_parser__test__always_err_decoder() -> Decoder {
    #[no_mangle]
    pub extern "C" fn lib_ruby_parser__test__always_err_decoder_fn(
        encoding: StringBlob,
        input: ByteListBlob,
    ) -> DecoderResultBlob {
        // do cleanup
        drop(String::from(encoding));
        drop(Vec::<u8>::from(input));
        // and return constant output
        DecoderResultBlob::from(DecoderResult::Err(InputError::DecodingError(String::from(
            "always_err",
        ))))
    }
    Decoder {
        f: lib_ruby_parser__test__always_err_decoder_fn,
    }
}

#[cfg(feature = "tests")]
#[no_mangle]
pub extern "C" fn lib_ruby_parser__test__some_always_ok_decoder() -> MaybeDecoderBlob {
    MaybeDecoderBlob::from(Some(lib_ruby_parser__test__always_ok_decoder()))
}

#[cfg(feature = "tests")]
#[no_mangle]
pub extern "C" fn lib_ruby_parser__test__none_decoder() -> MaybeDecoderBlob {
    MaybeDecoderBlob::from(None)
}