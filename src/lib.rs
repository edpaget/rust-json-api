extern crate hyper;

use std::io::Read;
use std::collections::HashMap;

use hyper::Client;
use hyper::net::HttpConnector;
use hyper::header::Connection;
use hyper::header::ConnectionOption;

pub struct ResourceCollection {
    name: String,
    link_template: String
}

pub struct JSONApiClient<C> {
    client: C, 
    entrypoint: &'static str,
}

impl<'a> JSONApiClient<Client<HttpConnector<'a>>> {
    pub fn new(entrypoint: &'static str) -> JSONApiClient<Client<HttpConnector<'a>>> {
        JSONApiClient::with_client(Client::new(), entrypoint)
    }
}

impl<C> JSONApiClient<C> {
    pub fn with_client(client: C, entrypoint: &'static str) -> JSONApiClient<C> {
        JSONApiClient {
            client: client,
            entrypoint: entrypoint,
        }
    }

    pub fn fetch(self, resource_name: 'static str) -> {
        
    }
}

#[test]
fn create_client() {
    let client = JSONApiClient::new("https://panoptes.zooniverse.org/api");
    assert!(true, "Client was created")
}

#[test]
fn it_works() {
    let mut client = Client::new();
    let mut res = client.get("http://www.isitchristmas.com/")
        .header(Connection(vec![ConnectionOption::Close]))
        .send().unwrap();

    assert_eq!(res.status, hyper::Ok);
}
