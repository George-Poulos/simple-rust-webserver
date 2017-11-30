# Simple Rust WebServer

This repo served as a learning experience for the rust language and attempts to create a simple rust webserver. This project will be tentatively updated.


To run, go into the git directory and run this command : 


    cargo build
    cargo run --example <name of example file>


To start a webserver, include the lib in your main .rs file

Then type : 


    let <some var name> = WebServer::new(String::from("<hostaddress:port>"));
    <some var name>.wait_for_request(); //event handler that waits for requests