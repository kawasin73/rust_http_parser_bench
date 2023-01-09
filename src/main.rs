#![feature(test)]

extern crate test;

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{ptr, io::Cursor};

    use test::Bencher;

    /// benchmark HTTP request from https://github.com/h2o/picohttpparser/blob/master/bench.c
    const REQUEST: &[u8] =
        b"GET /wp-content/uploads/2010/03/hello-kitty-darth-vader-pink.jpg HTTP/1.1\r\n\
        Host: www.kittyhell.com\r\n\
        User-Agent: Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10.6; ja-JP-mac; rv:1.9.2.3) Gecko/20100401 Firefox/3.6.3 \
        Pathtraq/0.9\r\n\
        Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\r\n\
        Accept-Language: ja,en-us;q=0.7,en;q=0.3\r\n\
        Accept-Encoding: gzip,deflate\r\n\
        Accept-Charset: Shift_JIS,utf-8;q=0.7,*;q=0.7\r\n\
        Keep-Alive: 115\r\n\
        Connection: keep-alive\r\n\
        Cookie: wp_ozh_wsa_visits=2; wp_ozh_wsa_visit_lasttime=xxxxxxxxxx; \
        __utma=xxxxxxxxx.xxxxxxxxxx.xxxxxxxxxx.xxxxxxxxxx.xxxxxxxxxx.x; \
        __utmz=xxxxxxxxx.xxxxxxxxxx.x.x.utmccn=(referral)|utmcsr=reader.livedoor.com|utmcct=/reader/|utmcmd=referral\r\n\
        \r\n";

    struct Callback;

    #[bench]
    fn bench_http_parser(b: &mut Bencher) {
        use http_parser::HttpParser;
        use http_parser::HttpParserType;
        use http_parser::HttpParserCallback;

        impl HttpParserCallback for Callback {
        }

        let mut cb = Callback{};

        b.iter(|| {
            let mut parser = HttpParser::new(HttpParserType::Request);
            let size = parser.execute(&mut cb, REQUEST);
            assert_eq!(size, REQUEST.len());
        });
        b.bytes = REQUEST.len() as u64;
    }

    #[bench]
    fn bench_dumb_http_parser(b: &mut Bencher) {
        use dumb_http_parser::HttpParser;

        b.iter(|| {
            let mut parser = HttpParser::new(REQUEST);
            parser.parse();
        });
        b.bytes = REQUEST.len() as u64;
    }

    #[bench]
    fn bench_http_box(b: &mut Bencher) {
        use http_box::http1::Parser;
        use http_box::fsm::Success;
        use http_box::http1::HttpHandler;
        impl HttpHandler for Callback {
        }

        let mut parser = Parser::new();
        let mut cb = Callback{};

        b.iter(|| {
            match parser.resume(&mut cb, REQUEST).unwrap() {
                Success::Finished(size) => {
                    assert_eq!(size, REQUEST.len());
                },
                other => {
                    panic!("unexpected: {:?}", other);
                }
            };
            parser.reset();
        });
        b.bytes = REQUEST.len() as u64;
    }

    #[bench]
    fn bench_thhp(b: &mut Bencher) {
        use thhp::Request;
        use thhp::Status;

        let mut headers = Vec::<thhp::HeaderField>::with_capacity(32);
        b.iter(|| {
            headers.clear();
            match Request::parse(REQUEST, &mut headers).unwrap() {
                Status::Complete((_, size)) => {
                    assert_eq!(size, REQUEST.len());
                },
                other => {
                    panic!("unexpected: {:?}", other);
                }
            }
        });
        b.bytes = REQUEST.len() as u64;
    }

    #[bench]
    fn bench_stream_httparse(b: &mut Bencher) {
        use stream_httparse::streaming_parser::ReqParser;

        let mut parser = ReqParser::new_capacity(32);
        b.iter(|| {
            match parser.block_parse(REQUEST) {
                (true, None) => {},
                other => {
                    panic!("unexpected: {:?}", other);
                }
            }
            parser.finish().unwrap();
            parser.clear();
        });
        b.bytes = REQUEST.len() as u64;
    }

    #[bench]
    fn bench_http_pull_parser(b: &mut Bencher) {
        use http_pull_parser::Parser;

        b.iter(|| {
            let mut parser = Parser::request();
            let mut nsize = 0;
            while nsize < REQUEST.len() {
                let (_, size) = parser.next_token(Some(&REQUEST[nsize..]));
                nsize += size;
            }
        });
        b.bytes = REQUEST.len() as u64;
    }

    #[bench]
    fn bench_picohttpparser_sys(b: &mut Bencher) {
        use picohttpparser_sys::phr_parse_request;
        use picohttpparser_sys::phr_header;

        let mut method: *const libc::c_char = ptr::null();
        let mut method_len: usize = 0;
        let mut path: *const libc::c_char = ptr::null();
        let mut path_len: usize = 0;
        let mut minor_version: libc::c_int  = 0;
        let mut headers = [phr_header::default();32];
        let mut num_headers: usize = 32;
        b.iter(|| {
            num_headers = 32;
            let size = unsafe {
                phr_parse_request(
                    REQUEST.as_ptr() as *const libc::c_char,
                    REQUEST.len(),
                    &mut method,
                    &mut method_len,
                    &mut path,
                    &mut path_len,
                    &mut minor_version,
                    headers.as_mut_ptr(),
                    &mut num_headers, 0) as usize
            };
            assert_eq!(size, REQUEST.len());
        });
        b.bytes = REQUEST.len() as u64;
    }

    #[bench]
    fn bench_http_bytes(b: &mut Bencher) {
        use http_bytes::parse_request_header;
        let mut headers_buffer = vec![http_bytes::EMPTY_HEADER; 32];

        b.iter(|| {
            let (_, rest) = parse_request_header(REQUEST, &mut headers_buffer, Some(http_bytes::http::uri::Scheme::HTTP)).unwrap().unwrap();
            assert_eq!(rest.len(), 0);
        });
        b.bytes = REQUEST.len() as u64;
    }

    #[bench]
    fn bench_httparse(b: &mut Bencher) {
        use httparse::Request;
        use httparse::Status;

        let mut headers = [httparse::EMPTY_HEADER; 16];

        b.iter(|| {
            let mut req = Request::new(&mut headers);
            let status = req.parse(REQUEST).unwrap();
            match status {
                Status::Complete(size) => {
                    assert_eq!(size, REQUEST.len());
                },
                other => {
                    panic!("unexpected: {:?}", other);
                }
            }
        });
        b.bytes = REQUEST.len() as u64;
    }

    #[bench]
    fn bench_rhymuweb(b: &mut Bencher) {
        use rhymuweb::Request;

        b.iter(|| {
            let mut req = Request::new();
            let result = req.parse(REQUEST).unwrap();
            assert_eq!(result.consumed, REQUEST.len());
        });
        b.bytes = REQUEST.len() as u64;
    }

    #[bench]
    fn bench_saf_httparser(b: &mut Bencher) {
        use saf_httparser::request_from_bytes;

        b.iter(|| {
            let _ = request_from_bytes(REQUEST).unwrap();
        });
        b.bytes = REQUEST.len() as u64;
    }

    #[bench]
    fn bench_http_muncher(b: &mut Bencher) {
        use http_muncher::Parser;
        use http_muncher::ParserHandler;
        impl ParserHandler for Callback {}

        let mut cb = Callback{};

        b.iter(|| {
            let mut parser = Parser::request();
            let size = parser.parse(&mut cb, REQUEST);
            assert_eq!(size, REQUEST.len());
        });
        b.bytes = REQUEST.len() as u64;
    }

    #[bench]
    fn bench_milstian_http(b: &mut Bencher) {
        use milstian_http::request::Message;

        b.iter(|| {
            let msg = Message::from_tcp_stream(REQUEST);
            assert!(msg.is_some());
        });
        b.bytes = REQUEST.len() as u64;
    }

    #[bench]
    fn bench_rocket_http_hyper(b: &mut Bencher) {
        use rocket_http::hyper::h1::parse_request;
        use rocket_http::hyper::buffer::BufReader;

        b.iter(|| {
            let mut buf = BufReader::new(Cursor::new(REQUEST));
            let _ = parse_request(&mut buf).unwrap();
        });
        b.bytes = REQUEST.len() as u64;
    }

    #[bench]
    fn bench_http_tiny(b: &mut Bencher) {
        use http_tiny::HeaderStartLine;
        use http_tiny::HeaderFields;

        b.iter(|| {
            let mut buf = Cursor::new(REQUEST);
            let _ = HeaderStartLine::read(&mut buf).unwrap();
            let _ = HeaderFields::read(&mut buf).unwrap();
            assert_eq!(buf.position() as usize, REQUEST.len());
        });
        b.bytes = REQUEST.len() as u64;
    }

    // #[bench]
    // #[ignore = "pico is broken"]
    // fn bench_pico(b: &mut Bencher) {
    //     use pico::request::RequestParser;
    //     use pico::HEADER_EMPTY;
    //     use pico::ChunkReader;
    //     use pico::Chunks;

    //     struct RequestReader {
    //         read: usize,
    //     }

    //     impl ChunkReader<RequestReader> for RequestReader {
    //         fn read(self, buf: &mut [u8]) -> (Option<usize>, RequestReader) {
    //             let size = std::cmp::min(REQUEST.len() - self.read, buf.len());
    //             std::ptr::copy_nonoverlapping(REQUEST[self.read..].as_ptr(), buf.as_mut_ptr(), size);
    //             self.read += size;
    //             (Some(size), self)
    //         }
    //     }

    //     impl Chunks for RequestReader {
    //         type Reader: ChunkReader<RequestReader>;

    //         fn chunk<F>(self, f: F) where F: FnOnce(Self::Reader) {
    //             f(self);
    //         }
    //     }


    //     let mut buf = [0_u8; 4096];
    //     let mut headers = [HEADER_EMPTY; 32];

    //     b.iter(|| {
    //         let parser = RequestParser::new(&mut buf, &mut headers);
    //         parser.parse(Reader{read: 0}, |result, _, body| {
    //             let _ = result.unwrap();
    //             assert_eq!(body.len(), 0);
    //         })
    //     });
    // }

    #[bench]
    fn bench_uhttp_request(b: &mut Bencher) {
        use uhttp_request::RequestLine;
        use uhttp_request::Headers;

        b.iter(|| {
            let (_, rest) = RequestLine::new(REQUEST).unwrap();
            let mut headers = Headers::new(rest);
            while let Some(_) = headers.next() {}
            assert_eq!(headers.into_inner().len(), 0);
        });
        b.bytes = REQUEST.len() as u64;
    }
}
