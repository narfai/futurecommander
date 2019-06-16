
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

const { PassThrough } = require('stream');

class MessageFrame extends PassThrough {
    constructor(options = {}) {
        options.readableObjectMode = true;
        options.writableObjectMode = true;
        super(options);


        this.decode = options.decode;
        this._buffer = Buffer.alloc(0);

        this.tx_count = 0;
        this.rx_count = 0;
    }

    read(){
        this.pause();
        const message = this.decode(this._buffer);

        console.log(this.tx_count);
        if(message.len()) {
            this.tx_count++;
            this._buffer = this._buffer.slice(0, this._buffer.length - message.len());
            this.resume();
            this.push(message);
        }
    }

    write(chunk) {
        console.log(this.rx_count);
        this.rx_count++;
        this._buffer = Buffer.concat([this._buffer, chunk]);
        this.resume();
    }
}

module.exports = {
    MessageFrame
};
