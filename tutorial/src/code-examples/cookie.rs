extern crate oak_http_server;
use oak_http_server::Cookie;

fn main() {
    let mut cookie = Cookie::new("foo", "bar");

    cookie.set_http_only(true).set_secure(true);

    assert!(
        cookie
            == Cookie {
                name: String::from("foo"),
                value: String::from("bar"),
                domain: None,
                expires: None,
                http_only: true,
                path: None,
                same_site: None,
                secure: true,
            },
    )
}
