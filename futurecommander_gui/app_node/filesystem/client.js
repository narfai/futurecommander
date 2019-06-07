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

const { Response } = require('./response');
const { Request } = require('./request');


class FileSystemClient {
    constructor() {
        console.log('Create FileSystemClient');
        this.Response = Response;
        this.Request = Request;
        this.resolves = {};
        this.rejects = {};
        this.worker = new Worker('app_node/filesystem/worker.js');
        this.worker.onmessage = ({ data }) => {
            const response = new Response(data);
            try {
                const resolve = this.resolves[response.id];
                if (resolve) {
                    resolve(response)
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
        console.log('SENT' ,request);
        return new Promise((resolve, reject) => {
            this.subscribe(request.id, resolve, reject);
            this.worker.postMessage([request]);
        });
    }
}

module.exports = FileSystemClient;