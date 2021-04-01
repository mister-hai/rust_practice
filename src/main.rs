// we want to take JSON formatted to the spec
// from an arbitrary port
// set by json payload
// and run commands with it

// this is a server/client in one that can be a standalone application

//Basic stuff
use std::{char, collections::VecDeque};
use std::time::Duration;
use std::io::{self, Write};
use termcolor::{Color,ColorChoice,ColorSpec, 
                StandardStream,WriteColor};
//use serde_json::{};
use std::thread::{Thread,JoinHandle};
use threadpool::ThreadPool;

//Network stuff
use tokio::net::{TcpStream,TcpSocket,TcpListener};
use std::net::{SocketAddr,IpAddr,Ipv4Addr}; 
use std::io::prelude::*;
use std::process::Command;
use std::str::from_utf8;

// GLOBALS
static max_threads: i32 = 4;

//why do I have to do this people?
// This is a series of various string concatention functions
fn concat_format(text1:str , text2:str){
    format!("{}{}", text1 , text2)
}
fn concat_print(text1:str , text2:str){
    println!("{}{}", text1 , text2)
}
// this is a threading function , you feed it functions
// we are going to be performing asynchronous actions 
fn generic_threader(num_threads : i32 , process_to_thread : fn() ) {
    if num_threads < max_threads {
        let mut pool = ThreadPool::new(num_threads);
        process_to_thread
    } else {

    }
fn is_connection_authorized(auth_string:str){

}
// if attempted connection is explicitly requesting an encrypted session, 
// check auth then process into tunnel
fn handle_ssh(){

}

fn network_IO(server_port: i32 ,ip_addr: i32) {

    // accept connections and process them, spawning a new thread for each one
    // this is a "listener" that waits on a port for connections
    // 0.0.0.0 is "this machine"
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server listening on port 3333");

    // if loop watever {
    //this is a loop that does an action for each connection attempt
    //tcpstream is a "stream of data" from an open socket
    // listener.incoming() is the things with the stream
    // peer_addr : returns IP of entity attempting connection
        // println!("New connection: {}", stream.peer_addr().unwrap());
    // we check the auth with each new connection
            //is_connection_authorized()
            // connection succeeded
            // we call our function created above
            // to do things with the data
            //generic_threader(1,handle_client(stream));
            
        //Err(e){
        //    println!("Error: {}", e);
            /* connection failed */
        //}
    }
    // close the socket server
    //    drop(listener);
}
// Framework for the JSON command schema
// This creates a "type" of object called "command_struct"
struct command_struct{
    name           : str,
    command        : str,
    info_message   : str,
    success_message: str,
    failure_message: str
}
// implements a functionality for the type we just created
// throwing an error if it fails
// takes a REFERENCE to self
// takes JSON
// Points to self, does not take itself as an object
// indicates the return type is a Command
impl command_struct{
    fn new(&self, json_input) -> Command {
        // now we can assign to self
        Command::new("sh");
        // if command successful
        println!(self.success_message);
        //else
        println!(self.failure_message);
    }
}
//holds Command_structs as a set
// literally just a container with a add/remove function
struct command_set{
    set : VecDeque<>
}
fn whatever(){
    let new_command = Command::new("sh" );
    new_command.arg("-c");
    new_command.arg("");
    new_command.output();
    new_command.expect("failed");

    let result = new_command.stdout;

}
// helper function to translate JSON from the wire into
// the struct
fn json_to_command_struct(){

}
// function to extract json parameters from new connections
fn get_json_from_wire(){

}
// function to deal with the client
fn handle_connection(mut stream: tokio::stream::TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            //stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn untyped_example(json_string: str) -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user
    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.

    Ok(())
}
fn main(){
    pass

}