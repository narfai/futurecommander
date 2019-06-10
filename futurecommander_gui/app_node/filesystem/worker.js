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


const { spawn } = require('child_process');

const addon = require('pkg/futurecommander_gui');

const { Request } = require('./request');

const { Socket } = require('net');

const uniqid = require('uniqid');


const { PassThrough } = require('stream');

class Message {
    constructor(header, payload){
        this.header = header;
        this.payload = payload;
        this.identifier = uniqid.time();
    }
}

class MessageFrame extends PassThrough {
    constructor(options) {
        options.readableObjectMode = true;
        options.writableObjectMode = true;
        super(options);
        this._buffer = Buffer.alloc(0);

        this._codec = options.codec;

        this.tx_count = 0;
        this.rx_count = 0;
    }

    read(){
        this.pause();
        const message = this._codec.decode(this._buffer);

        if(message.len()) {
            this.tx_count++;
            this._buffer = this._buffer.slice(message.len());
            this.resume();
            console.log(this.tx_count);
            this.push(new Message(message.header(), message.parse()));
        }
    }

    write(chunk) {
        this.rx_count++;
        this._buffer = Buffer.concat([chunk, this._buffer]);
        this.resume();
    }
}

class FileSystemWorker {
    constructor() {
        this._socket = new Socket();
        this._codec = new addon.ProtocolCodec();

        this._socket.connect(7842, '127.0.0.1', () => {
            console.log('Connected');

            for (let i = 0; i < 10000; i++) { // Always block at 405 - may it hit some TCP critical value
                setTimeout(() => this._socket.write(this._codec.read_dir()), 0); // 405 simultaneous requests is ok for a client
            }
        });

    }

    emit(request) {
        //@deprecate
    }

    send(message) {
        //TODO send addon encoded messages through the pipe
        // this.socket.write(addon.encode(message))
    }

    listen(){

        const framed = this._socket.pipe(new MessageFrame({ codec: this._codec }));

        framed.on('connect', () => {
            console.log('connect !' );
        });

        framed.on('drain', (data) => {
            console.log('DRAIN', data);
        });

        framed.on('data', (data) => {
            console.log('DATA CALL', data);
            // TODO postMessage then transform

        });

        framed.on('close', function() {
            console.log('Connection closed');
        });
    }

    close() {
        this.close_count += 1;
        this.filesystem.unref();
        this.filesystem = null;
    }
}

let worker = new FileSystemWorker();

worker.listen();

onmessage = function(e) {
    worker.emit(e.data[0]);//@deprecate
};
