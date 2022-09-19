/*
ORBITAL GNC CONTROL UI

Function: Provide a visual UI for controlling the GNC Zephyr system.

User Input --
    Desired commands for Zephyr to Execute

Other Input --
    Sensor data from IMU (?)

UI Output --
    Currently running command
    Sensor data (?)

Other Output --
    Command for rocket to run (through terminal?)
*/

// use std::{io, thread, time::Duration};
// use tui::{
//     backend::CrosstermBackend,
//     widgets::{Widget, Block, Borders},
//     layout::{Layout, Constraint, Direction},
//     Terminal
// };
//
// use crossterm::{
//     event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };
//
// use std::net::{TcpListener, TcpStream, Shutdown};
// use std::io::{Read, Write};
//
// fn handle_client(mut stream: TcpStream) {
//     let mut data = [0 as u8; 50]; // using 50 byte buffer
//     while match stream.read(&mut data) {
//         Ok(size) => {
//             // echo everything!
//             stream.write(&data[0..size]).unwrap();
//             true
//         },
//         Err(_) => {
//             println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
//             stream.shutdown(Shutdown::Both).unwrap();
//             false
//         }
//     } {}
// }

fn main() -> Result<()> {
    let app = Rc::new(RefCell::new(App::new())); // TODO app is useless for now
    start_ui(app)?;
    Ok(())
}

// let listener = TcpListener::bind("0.0.0.0:65432").unwrap();
// // accept connections and process them, spawning a new thread for each one
// println!("Server listening on port 65432");
// for stream in listener.incoming() {
//     match stream {
//         Ok(stream) => {
//             println!("New connection: {}", stream.peer_addr().unwrap());
//             thread::spawn(move|| {
//                 // connection succeeded
//                 handle_client(stream)
//             });
//         }
//         Err(e) => {
//             println!("Error: {}", e);
//             /* connection failed */
//         }
//     }
// }
// // close the socket server
// drop(listener);
