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

const { Message } = require('./message');
const EventEmitter = require('events');

class FileSystemClient extends EventEmitter {
    constructor(options = {}) {
        options.allowHalfOpen = false;
        options.readableObjectMode = true;
        options.writableObjectMode = true;
        options.readableFlowing = false;
        super(options);

        this.Message = Message;
        this.worker = null;
        if(this.worker === null) {
            this.listen()
        }
    }
    listen() {
        this.worker = new Worker('app_node/filesystem/worker.js');
        this.worker.onmessage = ({data}) => {
            this.emit('in_message', new Message(data));
        };

        this.on('out_message', (message) => {
            this.worker.postMessage([message]);
        });

        this.worker.onerror = ((error) => {
            process.nextTick(() => this.emit('error', error));
        })
    }

    message(user_message) {
        return new Message(user_message);
    }
}

module.exports = FileSystemClient;
