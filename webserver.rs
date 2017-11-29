/*
 * Author: George Poulos
 * 
 * Description : This demonstrates a simple webserver in Rust
 */

 use std::io::{Read,Write, BufReader, BufRead};
 use std::net::{TcpListener,TcpStream};
 use std::fs::File;
 use std::thread;

 #[derive(Clone)]
 struct WebServer{
    host : String
 }

 /**
  * 'Class' in rust for a webserver
  */
 impl WebServer{
    pub fn new(host_str : String) -> WebServer{
        WebServer{host : host_str}
     }

    /**
     * Function that waits for a client request and serves a page/data
     */
    pub fn wait_for_request(&self){
        let self_clone = self.clone();
        let listener = TcpListener::bind(self_clone.host).unwrap();
        for stream in listener.incoming() {
            match stream{
                Ok(stream) =>{
                    let self_clone2 = self.clone();
                    let child = thread::spawn(move || {
                        println!("Connection Established!");
                        self_clone2.handle_request(&stream);
                    });
                    let child_ret = child.join();
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
    pub fn handle_request(&self, stream : &TcpStream){
        let mut reader = BufReader::new(stream);
        for line in reader.by_ref().lines() {
            if line.unwrap() == "" {
                break;
            }
        }
        self.send_response(reader.into_inner(), String::from("test.html"));
    }


    /**
    * Function that sends a response to a given stream ref. by sending the
    * contents of a file.
    */
    pub fn send_response(&self, mut stream: &TcpStream, file_name : String) {
        let mut file = File::open(file_name).expect("file not found");
        let mut contents = String::new();
        let mut resp_code = String::from("HTTP/1.1 200 OK\n\n");
        file.read_to_string(&mut contents).expect("Bad File was given!");
        resp_code.push_str(& contents);
        stream.write_all(resp_code.as_bytes()).unwrap();
    }
 }

 fn main(){
    let ws = WebServer::new(String::from("127.0.0.1:5432"));
    ws.wait_for_request();
 }
 
 