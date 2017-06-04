// Original file: "InputStream.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Data::Word;
// use Data::ByteString;
// use ByteString;
// use Data::ByteString;
// use Data::ByteString::Char8;
// use Data::Char;

pub type InputStream = ByteString;

pub type InputStream = String;

pub fn countLines() -> isize {
    match () {
        () => {
            length(BSC::lines)
        },
        () => {
            length(BSC::lines)
        },
    }
}

pub fn inputStreamEmpty() -> bool {
    match () {
        () => {
            BSW::null
        },
        () => {
            BSW::null
        },
    }
}

pub fn inputStreamFromString() -> InputStream {
    match () {
        () => {
            BSC::pack
        },
        () => {
            BSC::pack
        },
    }
}

pub fn inputStreamToString() -> String {
    match () {
        () => {
            BSC::unpack
        },
        () => {
            BSC::unpack
        },
    }
}

pub fn readInputStream() -> IO<InputStream> {
    match () {
        () => {
            BSW::readFile
        },
        () => {
            BSW::readFile
        },
    }
}

pub fn takeByte(_0: InputStream) -> (Word8, InputStream) {
    match (_0) {
        bs => {
            seq(BSW::head(bs), (BSW::head(bs), BSW::tail(bs)))
        },
        bs => {
            seq(BSW::head(bs), (BSW::head(bs), BSW::tail(bs)))
        },
    }
}

pub fn takeChar(_0: InputStream) -> (Char, InputStream) {
    match (_0) {
        bs => {
            seq(BSC::head(bs), (BSC::head(bs), BSC::tail(bs)))
        },
        bs => {
            seq(BSC::head(bs), (BSC::head(bs), BSC::tail(bs)))
        },
    }
}

pub fn takeChars(_0: isize, _1: InputStream) -> Vec<Char> {
    match (_0, _1) {
        (n, bstr) => {
            BSC::unpack(BSC::take(n, bstr))
        },
        (n, __str) => {
            BSC::unpack(BSC::take(n, bstr))
        },
    }
}



