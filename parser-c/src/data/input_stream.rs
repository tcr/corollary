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

pub fn countLines<a>() -> isize {
    match () {
        () => {
            length(BSC::lines)
        },
        () => {
            length(lines)
        },
    }
}

pub fn inputStreamEmpty<a>() -> bool {
    match () {
        () => {
            BSW::null
        },
        () => {
            null
        },
    }
}

pub fn inputStreamFromString<a>() -> InputStream {
    match () {
        () => {
            BSC::pack
        },
        () => {
            id
        },
    }
}

pub fn inputStreamToString<a>() -> String {
    match () {
        () => {
            BSC::unpack
        },
        () => {
            id
        },
    }
}

pub fn readInputStream<a>() -> IO<InputStream> {
    match () {
        () => {
            BSW::readFile
        },
        () => {
            readFile
        },
    }
}

pub fn takeByte<a>(_0: InputStream) -> (Word8, InputStream) {
    match (_0) {
        bs => {
            seq(BSW::head(bs), (BSW::head(bs), BSW::tail(bs)))
        },
        bs => {
            /* Expr::Error */ Error
        },
    }
}

pub fn takeChar<a>(_0: InputStream) -> (Char, InputStream) {
    match (_0) {
        bs => {
            seq(BSC::head(bs), (BSC::head(bs), BSC::tail(bs)))
        },
        bs => {
            (head(bs), tail(bs))
        },
    }
}

pub fn takeChars<a>(_0: isize, _1: InputStream) -> Vec<Char> {
    match (_0, _1) {
        (n, bstr) => {
            BSC::unpack(BSC::take(n, bstr))
        },
        (n, __str) => {
            take(n, __str)
        },
    }
}



