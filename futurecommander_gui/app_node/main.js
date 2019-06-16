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

class NodeApplication {
    constructor() { //Node dependencies - Main ChromeApp thread
        // this.filesystem_client = new FileSystemClient();
    }

    run({ nw }) {
        nw.Window.open(
            'app_web/index.html',
            {
                'id': 'main',
                new_instance: false
            },
            function(win) {
                const filesystem_client = new FileSystemClient();
                win.on('closed', function () {
                    filesystem_client.close();
                    win = null;
                });

                win.on('loaded', () => {
                    win.showDevTools();
                    const Application = nw.require('app_web/index.js');
                    const app = new Application(
                        win.window,
                        filesystem_client
                    );
                    filesystem_client.listen();
                    app.run();
                });
            }
        );

    }
}

module.exports = {
    NodeApplication
};
