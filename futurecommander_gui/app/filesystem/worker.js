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

class FileSystemWorker {
    constructor() {
        this.filesystem = null;
    }

    emit(id, request) {
        console.log('REQUEST', id, request);
        switch (request.type) {
            case 'LIST':
                this.filesystem.stdin.write(addon.list(id, request.path));
                break;
            default:
                console.log('unknown request', request);
                break;
        }
    }

    listen() {
        try {
            this.filesystem = spawn(
                '../target/debug/futurecommander',
                ['daemon'],
                {
                    detached: true,
                    stdio: 'pipe',
                    env: {
                        'RUST_BACKTRACE': 1
                    }
                }
            );

            this.filesystem.stdout.on('data', (response) => {
                let res = addon.decode(response);

                postMessage({
                    id: res.id(),
                    status: res.status(),
                    kind: res.kind()
                });
            });

            this.filesystem.stderr.on('data', (data) => {
                global.console.log(`stderr: ${data}`);
            });

            this.filesystem.on('close', (code) => {
                global.console.log(`child process exited with code ${code}`);
                this.close();
                this.listen();
            });

            this.filesystem.on('error', (error) => {
                global.console.log(`child process error ${error}`);
            });
        } catch (err) {
            global.console.log(`${err}`);
        }
    }

    close() {
        this.filesystem.unref();
        this.filesystem = null;
    }
}

let worker = new FileSystemWorker();

onmessage = function(e) {
    worker.listen();
    worker.emit(e.data[0], e.data[1]);
};
