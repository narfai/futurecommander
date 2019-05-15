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

// const FileSystemClient = new require('./filesystem/client');

// const { Request } = require('./filesystem/request');

class NodeApplication {
    constructor() { //Node dependencies - Main ChromeApp thread
        // this.filesystem_client = new FileSystemClient();
    }

    run({ nw }) {
        nw.Window.open(
            'app_web/index.html',
            {
                'id': 'main',
                new_instance: false,
                inject_js_end: 'app_web/index.js'
            },
            function(win) {
                win.on('loaded', () => {
                    win.showDevTools();
                });
            }
        );

    }
}

module.exports = {
    NodeApplication
};
