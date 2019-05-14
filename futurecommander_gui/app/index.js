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


const { Request } = require('./filesystem/api');

//TODO tcomb, redux, proper promises, shared enums, tests, linter & whole QA
//TODO easy ui loader

module.exports = class Application {
    constructor() {
        this.filesystem_client = new FileSystemClient();
    }

    run() {
        // Create an empty context menu
        var menu = new nw.Menu();

        // Add some items with label
        menu.append(new nw.MenuItem({
            label: 'LIST',
            click: () => {
                this.filesystem_client.send(Request.list('/home/narfai/tmp'))
                    .then((response) => {
                        console.log(response);
                    });
            }
        }));

        // menu.append(new nw.MenuItem({
        //     label: 'TEST ERROR',
        //     click: function() {
        //         // filesystem.stdin.write("test_error\n");
        //     }
        // }));
        //
        // menu.append(new nw.MenuItem({type: 'separator'}));
        // menu.append(new nw.MenuItem({
        //     label: 'Exit',
        //     click: function() {
        //         // filesystem.stdin.write("exit\n");
        //     }
        // }));

        // Hooks the "contextmenu" event
        document.body.addEventListener('contextmenu', function (ev) {
            // Prevent showing default context menu
            ev.preventDefault();
            // Popup the native context menu at place you click
            menu.popup(ev.x, ev.y);

            return false;
        }, false);
    }
};
