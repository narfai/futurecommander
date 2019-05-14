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

const { Response } = require('./api');


module.exports = class FileSystemClient {
    constructor() {
        this.resolves = {};
        this.rejects = {};
        this.worker = new Worker('./app/filesystem/worker.js');
        this.worker.onmessage = ({ data }) => {
            const response = new Response(data);
            try {
                const resolve = this.resolves[response.id];
                if (resolve) {
                    resolve(response.result())
                }
            } catch(error) {
                const reject = this.rejects[response.id];
                if (reject) {
                    reject({ response, error });
                }
            }
            this.unsubscribe(response.id)
        }
    }

    subscribe(id, resolve, reject) {
        this.resolves[id] = resolve;
        this.rejects[id] = reject;
    }

    unsubscribe(id) {
        if (typeof this.resolves[id] !== undefined) {
            delete this.resolves[id];
        }
        if (typeof this.rejects[id] !== undefined) {
            delete this.rejects[id];
        }
    }

    send(request) {
        return new Promise((resolve, reject) => {
            this.subscribe(request.id, resolve, reject);
            this.worker.postMessage([request]);
        });
    }
};
