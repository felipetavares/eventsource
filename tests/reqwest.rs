extern crate eventsource;
extern crate reqwest;

use eventsource::reqwest::Client;
use reqwest::Url;
use std::time::Duration;

use server::Server;
mod server;

fn server() -> Server {
    let s = Server::new();
    s.receive("\
GET / HTTP/1.1\r\n\
Host: 127.0.0.1:$PORT\r\n\
User-Agent: reqwest/0.4.0\r\n\
Accept: text/event-stream\r\n\
\r\n");
    return s;
}

#[test]
fn simple_events() {
    let s = server();
    s.send("HTTP/1.1 200 OK\r\n\
\r\n\
id: 42\r\n\
event: foo\r\n\
data: bar\r\n\
\r\n\
event: bar\n\
: comment\n\
data: baz\n\
\n");

    println!("url: {}", s.url("/"));
    let mut client = Client::new(Url::parse(&s.url("/")).unwrap()).unwrap();

    let event = client.next().unwrap().unwrap();
    assert_eq!(event.id, Some("42".into()));
    assert_eq!(event.event_type, Some("foo".into()));
    assert_eq!(event.data, "bar\n");

    let event = client.next().unwrap().unwrap();
    assert_eq!(event.id, None);
    assert_eq!(event.event_type, Some("bar".into()));
    assert_eq!(event.data, "baz\n");
}

#[test]
fn retry() {
    let s = server();
    s.send("HTTP/1.1 200 OK\r\n\
\r\n\
retry: 42\r\n\
data: bar\r\n\
\r\n");

    println!("url: {}", s.url("/"));
    let mut client = Client::new(Url::parse(&s.url("/")).unwrap()).unwrap();
    let event = client.next().unwrap().unwrap();
    assert_eq!(event.data, "bar\n");
    assert_eq!(client.retry, Duration::from_millis(42));
}
