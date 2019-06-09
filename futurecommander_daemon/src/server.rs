/*
 * Copyright 2019 François CADEILLAN
 *
 * This file is part of FutureCommander.
 *
 * FutureCommander is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * FutureCommander is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with FutureCommander.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::{
    io::{
        prelude::*,
        Write,
        Stdin,
        Stdout,
        Stderr
    },
    sync::{
        Arc,
        Mutex
    }
};

use tokio::{
    io,
    net::{ TcpListener, TcpStream },
    prelude::{
        Async,
        Poll,
//        stream::{ },
        future::{ Either, ok, lazy },
        *
    },
    codec::{
        Framed
    }
};

use crate::{
    errors::DaemonError,
    message::{
        ProcessMessage,
        MessageCodec,
        Message
    }
};

use futurecommander_filesystem::{
    Container
};

pub type State = Arc<Mutex<Container>>;

pub fn process(state: State, socket: TcpStream){
    let (tx, rx) = Framed::new(socket, MessageCodec::default()).split();

    let task = tx.send_all(
        rx.and_then(move |message|{
            println!("Incoming message {:?}", message);
            ProcessMessage::new(message, state.clone())
                .then(|message| {
                    println!("Reply {:?}", message);
                    message
                })
        })
    ).then(|res| {
        if let Err(e) = res {
            println!("failed to process connection; error = {:?}", e);
        }

        Ok(())
    });

    tokio::spawn(task);
}

pub fn listen() {
    let addr = "127.0.0.1:7842".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let mut state : State = Arc::default();
    let server = listener.incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            process(Arc::clone(&state), socket);
            Ok(())
        });

    println!("server running on localhost:7842");

    tokio::run(server);
}
