// Original file: "InputStream.hs"
// File auto-generated using Corollary.

#[macro_use]
use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Data::Word;
// use Data::ByteString;
// use ByteString;
// use Data::ByteString;
// use Data::ByteString::Char8;
// use Data::Char;

pub type InputStream = ByteString;

pub fn takeByte(bs: InputStream) -> (Word8, InputStream) {
    seq(BSW::head(bs), (BSW::head(bs), BSW::tail(bs)))
}

pub fn takeChar(bs: InputStream) -> (char, InputStream) {
    seq(BSC::head(bs), (BSC::head(bs), BSC::tail(bs)))
}

pub fn inputStreamEmpty() -> bool {
    BSW::null
}

pub fn takeChars(n: isize, bstr: InputStream) -> Vec<char> {
    BSC::unpack(BSC::take(n, bstr))
}

pub fn readInputStream() -> IO<InputStream> {
    BSW::readFile()
}

pub fn inputStreamToString(bs: InputSstream) -> String {
    BSC::unpack(bs)
}

pub fn inputStreamFromString(s: String) -> InputStream {
    BSC::pack(s)
}

pub fn countLines() -> isize {
    length(BSC::lines)
}
