pub const _DEBUG: bool = true;

pub const ADDRESS: &str = "127.0.0.1:7878";

pub const NUM_CPU: usize = 4;
pub const SLEEP_SECS: u64 = 5;

pub const HELLO_HTML: &str = "templates/hello.html";
pub const _SLEEP_HTML_COUNTER: &str = "templates/sleep_counter.html";
pub const SLEEP_HTML: &str = "templates/sleep.html";
pub const NOT_FOUND_404_HTML: &str = "templates/404.html";

pub const GET_ROOT_URI: &str = "GET / HTTP/1.1";
pub const GET_SLEEP_URI: &str = "GET /sleep HTTP/1.1";

pub const STATUS_200_OK: &str = "HTTP/1.1 200 OK";
pub const STATUS_404_NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND";
