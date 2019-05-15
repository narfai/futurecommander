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

const FileSystemClient = require('./filesystem/client');


const { Request } = require('./filesystem/request');

//TODO tcomb, redux, proper promises, shared enums, tests, linter & whole QA
//TODO easy ui loader

module.exports = class Application {
    constructor() {
        this.filesystem_client = new FileSystemClient();
    }

    run() {
        var menu = new nw.Menu();

        menu.append(new nw.MenuItem({
            label: 'LIST',
            click: () => {
                this.filesystem_client.send(Request.list({ path: '/home/narfai/tmp' }))
                    .then((response) => {
                        console.log(response);
                    });
            }
        }));

        menu.append(new nw.MenuItem({
            label: 'STATUS',
            click: () => {
                this.filesystem_client.send(Request.status({ path: '/home/narfai/tmp' }))
                    .then((response) => {
                        console.log(response);
                    });
            }
        }));

        document.body.addEventListener('contextmenu', function (ev) {
            ev.preventDefault();
            menu.popup(ev.x, ev.y);

            return false;
        }, false);
    }
};
