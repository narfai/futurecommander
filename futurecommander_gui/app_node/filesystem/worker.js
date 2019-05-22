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

class FileSystemWorker {
    constructor() {
        this.filesystem = null;
        this.close_count = 0;
    }

    emit(request) {
        console.log('WORKER REQUEST', addon.request(new Request(request)));
        this.filesystem.stdin.write(
            addon.request(
                new Request(request)
            )
        );
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
                console.log('RESPONSE', response);
                postMessage(
                    addon.decode(response)
                );
            });

            this.filesystem.stderr.on('data', (data) => {
                global.console.log(`stderr: ${data}`);
            });

            this.filesystem.on('close', (code) => {
                global.console.log(`child process exited with code ${code}`);
                this.close();
                if(this.close_count > 5) {
                    global.console.log(`restart child process`);
                    this.listen();
                }
            });

            this.filesystem.on('error', (error) => {
                global.console.log(`child process error ${error}`);
            });
        } catch (err) {
            global.console.log(`${err}`);
        }
    }

    close() {
        this.close_count += 1;
        this.filesystem.unref();
        this.filesystem = null;
    }
}

let worker = new FileSystemWorker();

onmessage = function(e) {
    worker.listen();
    worker.emit(e.data[0]);
};
