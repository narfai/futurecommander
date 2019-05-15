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
            'node/web/index.html',
            {
                'id': 'main',
                new_instance: false,
                inject_js_end: 'node/web/main.js'
            },
            // function (win) { //Browser context - Same node context - Same thread until new_instance: true
            //     const mithril = nw.require('mithril');
            //     const { WebApplication } = nw.require('application/webapp/main.js');
            //
            //     console.log(win);
            //     win.on('loaded', () => {
            //         win.showDevTools();
            //         mithril.mount(win.document.body, () => new WebApplication({ 'attrs': {}}))
            //     });
            // }
        );

    }
}

module.exports = {
    NodeApplication
};
