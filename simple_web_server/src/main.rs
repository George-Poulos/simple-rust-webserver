/*
 * Author: George Poulos
 *
 * Description : This demonstrates a simple webserver in Rust
 */

use std::io::{Read, Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};
use std::fs::File;
use std::thread;
use std::env;

#[derive(Clone)]
pub struct WebServer {
    host: String,
    pub public_path: String,
}

 /**
  * 'Class' in rust for a webserver
  */
impl WebServer {
    pub fn new(host_str: String) -> WebServer {
        WebServer {
            host: host_str,
            public_path: env::current_dir().unwrap().to_str().unwrap().to_string(),
        }
    }
    pub fn new_with_path(host_str: String, path: String) -> WebServer {
        WebServer {
            host: host_str,
            public_path: path,
        }
    }

    /**
     * Function that waits for a client request and serves a page/data
     */
    pub fn wait_for_request(&self) {
        let self_clone = self.clone();
        let listener = TcpListener::bind(self_clone.host).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let self_clone2 = self.clone();
                    thread::spawn(move || { 
                        self_clone2.handle_request(&stream); 
                    });
                }
                Err(e) => {
                    println!("Connection Failed! -> {}", e);
                }
            }
        }
    }

    /**
     * Function that receives a TCP stream and handles that stream
     */
    pub fn handle_request(&self, stream: &TcpStream) {
        let mut reader = BufReader::new(stream);
        let mut get_req = String::new();
        reader.read_line(&mut get_req).expect("cannot read stream");
        let get_file = &get_req[4..].split(" ").collect::<Vec<&str>>();
        println!("{}", get_file[0]);
        for line in reader.by_ref().lines() {
            if line.unwrap() == "" {
                break;
            }
        }
        let mut final_str: String = self.public_path.to_owned();
        final_str.push_str(get_file[0]);
        self.send_response(reader.into_inner(), String::from(final_str));
    }


   /**
    * Function that sends a response to a given stream ref. by sending the
    * contents of a file.
    */
    pub fn send_response(&self, mut stream: &TcpStream, file_name: String) {
        let mut file = File::open(file_name).expect("file not found");
        let mut contents = String::new();
        let mut resp_code = String::from("HTTP/1.1 200 OK\n\n");
        file.read_to_string(&mut contents).expect(
            "Bad File was given!",
        );
        resp_code.push_str(&contents);
        stream.write_all(resp_code.as_bytes()).unwrap();
    }
}

fn main(){

}