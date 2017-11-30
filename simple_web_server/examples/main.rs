 extern crate simple_web_server;
 use simple_web_server::WebServer;
 /**
  * Using the webserver 'object'
  */
 fn main(){
    let ws = WebServer::new(String::from("127.0.0.1:5432"));
    ws.wait_for_request();
 }