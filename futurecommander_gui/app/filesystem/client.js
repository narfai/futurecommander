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

const uniqid = require('uniqid');

module.exports = class FileSystemClient {
    constructor() {
        this.resolves = {};
        this.rejects = {};
        this.worker = new Worker('./app/filesystem/worker.js');
        this.worker.onmessage = ({ data: response }) => {
            console.log(response);
            let id = response.id;
            switch (response.status) {
                case 'success':
                    console.log(this.resolves, id);
                    const resolve = this.resolves[id.toString()];
                    if (resolve) {
                        resolve(response);
                    }
                    break;
                case 'fail':
                    const reject = this.rejects[id];
                    if (reject) {
                        reject(response);
                    }
                    break;
            }

            delete this.resolves[id];
            delete this.rejects[id];
        }
    }

    send(request) {
        return new Promise((resolve, reject) => {
            const id =
                Array.from(uniqid.process())
                    .reduce(
                        (acc, cur) => acc + cur.charCodeAt(0).toString(10),
                        '0'
                    );

            this.resolves[id] = resolve;
            this.rejects[id] = reject;
            this.worker.postMessage([id, request]);
        });
    }
};
