//! Original file: "InputStream.hs"
//! File auto-generated using Corollary.

use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Data::Word;
// use Data::ByteString;
// use ByteString;
// use Data::ByteString;
// use Data::ByteString::Char8;
// use Data::Char;

pub fn countLines() -> isize {
    match () {
        () => {
            length(BSC::lines)
        },
        () => {
            length(lines)
        },
    }
}

pub fn inputStreamEmpty() -> bool {
    match () {
        () => {
            BSW::null
        },
        () => {
            null
        },
    }
}

pub fn inputStreamFromString() -> InputStream {
    match () {
        () => {
            BSC::pack
        },
        () => {
            id
        },
    }
}

pub fn inputStreamToString() -> String {
    match () {
        () => {
            BSC::unpack
        },
        () => {
            id
        },
    }
}

pub fn readInputStream() -> IO<InputStream> {
    match () {
        () => {
            BSW::readFile
        },
        () => {
            readFile
        },
    }
}

pub fn takeByte(_0: InputStream) -> (Word8, InputStream) {
    match (_0) {
        bs => {
            seq(BSW::head(bs), (BSW::head(bs), BSW::tail(bs)))
        },
        bs => {
            /* Expr::Error */ Error
        },
    }
}

pub fn takeChar(_0: InputStream) -> (Char, InputStream) {
    match (_0) {
        bs => {
            seq(BSC::head(bs), (BSC::head(bs), BSC::tail(bs)))
        },
        bs => {
            (head(bs), tail(bs))
        },
    }
}

pub fn takeChars(_0: isize, _1: InputStream) -> Vec<Char> {
    match (_0, _1) {
        (n, bstr) => {
            BSC::unpack(BSC::take(n, bstr))
        },
        (n, __str) => {
            take(n, __str)
        },
    }
}



