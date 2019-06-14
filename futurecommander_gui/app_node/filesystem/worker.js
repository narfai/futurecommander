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

class Message { // TODO move to upper namespace
    constructor(header, payload){
        this.header = header;
        this.payload = payload;
        this.identifier = uniqid.time();
    }
}

class MessageFrame extends PassThrough {
    constructor(options = {}) {
        options.readableObjectMode = true;
        options.writableObjectMode = true;
        super(options);

        this._buffer = Buffer.alloc(0);

        this.tx_count = 0;
        this.rx_count = 0;
    }

    read(){
        this.pause();
        const message = addon.Codec.decode(this._buffer);

        console.log(this.tx_count);
        if(message.len()) {
            this.tx_count++;
            this._buffer = this._buffer.slice(0, this._buffer.length - message.len());
            this.resume();
            this.push(new Message(message.header(), message.parse())); //TODO send ptr
        }
    }

    write(chunk) {
        console.log(this.rx_count);
        this.rx_count++;
        this._buffer = Buffer.concat([this._buffer, chunk]);
        this.resume();
    }
}

class FileSystemWorker { // TODO refactor it to be usable outside the worker
    constructor() {
        this._socket = new Socket();

        this._socket.on('disconnect', () => {
            console.log('disconnected !')
        });

        this._socket.on('close', () => {
            console.log('closed !')
        });

        this._socket.connect(7842, '127.0.0.1', () => { //TODO parametrize & promisify
            console.log('Connected');

            /**
             * TODO move to real benchmark
             * May it is even faster but there limited to javascript tick ( of whole worker )
             * kernel:              Linux 5.1.1 x86_64
             * cpu :                Intel(R) Core(TM) i7-5820K CPU @ 3.30GHz
             * ram :                32836740 kB DDR4
             * sent :               100 000 message
             * received:            100 000 message
             * time :               11 seconds
             * rate :               18181 message / second
             * average latency :    0.055 ms
             **/
            // for (let i = 0; i < 100000; i++) {
            //     setTimeout(() => {
            //         this.send('DirectoryOpen', { path: '/tmp2' })
            //     }, 0);
            // }
        });

    }

    emit(request) {
        //@deprecate
    }

    send(header, payload = {}) {
        let context = new addon.RustMessageContext(header);
        Object
            .keys(payload)
            .forEach((key) => context.set(key, payload[key]))
        ;

        this._socket.write(addon.Codec.encode(context))
    }

    listen(){

        const framed = this._socket.pipe(new MessageFrame());

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
        this.filesystem.unref();
        this.filesystem = null;
    }
}

let worker = new FileSystemWorker();

worker.listen();

onmessage = function(e) {
    worker.emit(e.data[0]);//@deprecate
};
