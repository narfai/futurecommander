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

// const { FileSystemClient }  = require('../app_node/filesystem/client');

const mithril = nw.require('mithril');

const { Provider, ActionCreator, Middleware } = nw.require('openmew-renderer');
const { list_filesystem, ready_state_promise, ready_state_redraw } = nw.require('./middleware');
const { applyMiddleware } = nw.require('redux');


const View = nw.require('./view');
const State = nw.require('./state');

module.exports = class Application {
    constructor(window, filesystem_client){
        this.provider = new Provider(mithril, 'Layout');

        View.connect(this.provider);

        this.store = State.connect(
            this.provider,
            applyMiddleware(
                list_filesystem(filesystem_client),
                ready_state_promise,
                Middleware.render(mithril, this.provider, window.document.body),
                ready_state_redraw(mithril)
            ),
            // nw.require('./state/mock.js'),
        );
    }

    run(){
        this.store.dispatch(ActionCreator.switch());
    }
};


