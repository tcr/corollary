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
    seq(BSW::head(bs.clone()), (BSW::head(bs.clone()), BSW::tail(bs)))
}

pub fn takeChar(bs: InputStream) -> (char, InputStream) {
    seq(BSC::head(bs.clone()), (BSC::head(bs.clone()), BSC::tail(bs)))
}

pub fn inputStreamEmpty(bs: InputStream) -> bool {
    BSW::null(bs)
}

pub fn takeChars(n: isize, bstr: InputStream) -> Vec<char> {
    BSC::unpack(BSC::take(n, bstr)).chars().collect()
}

pub fn takeChars_str(n: isize, bstr: InputStream) -> String {
    BSC::unpack(BSC::take(n, bstr)).chars().collect()
}
pub fn readInputStream(f: FilePath) -> InputStream {
    BSW::readFile(f)
}

pub fn inputStreamToString(bs: InputStream) -> String {
    BSC::unpack(bs)
}

pub fn inputStreamFromString(s: String) -> InputStream {
    BSC::pack(s)
}

pub fn countLines(bs: InputStream) -> isize {
    length(BSC::lines(bs))
}
