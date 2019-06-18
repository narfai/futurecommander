
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
const { Socket } = require('net');
const EventEmitter = require('events');

const { MessageFrame } = require('./frame');
const { Message } = require('../message');

class TcpMessageClient extends EventEmitter {
    constructor(options) {
        super(options);

        this._socket = new Socket();
        this.codec = options.codec;
        this.context = options.context;
    }

    send({ header, payload = {} }) {
        let context = new this.context(header);
        Object
            .keys(payload)
            .forEach((key) => context.set(key, payload[key]))
        ;

        this._socket.write(this.codec.encode(context))
    }

    listen(){
        this.listen_tcp();
        const framed = this._socket.pipe(new MessageFrame({
            decode: (bytes) => this.codec.decode(bytes)
        }));

        framed.on('connect', () => {
            console.debug('Framed client connected' );
        });

        framed.on('data', (data) => {
            postMessage(new Message({
                header: data.header(),
                payload: data.parse()
            }));

        });

        framed.on('closed', function() {
            console.debug('Framed connection closed');
        });
    }

    listen_tcp(){
        this._socket.on('disconnect', () => {
            console.debug('Socket disconnected !')
        });

        this._socket.on('closed', () => {
            console.debug('Socket closed !')
        });

        this._socket.connect(7842, '127.0.0.1', () => { //TODO parametrize & promisify
            console.debug('Socket connected');

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

    close() {
        this.filesystem.unref();
        this.filesystem = null;
    }
}

module.exports = {
    TcpMessageClient
};
