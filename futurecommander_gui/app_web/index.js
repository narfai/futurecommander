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

const { ActionCreator, Middleware } = nw.require('openmew-renderer');
const { send_filesystem, ready_state_promise, ready_state_redraw } = nw.require('./infrastructure/middleware');
const { applyMiddleware, createStore } = nw.require('redux');

const { ActionCreatorAdapter } = nw.require('./infrastructure/action_adapter');
const create_provider = nw.require('./infrastructure/provider');

module.exports = class Application {
    constructor(window, filesystem_client){
        this.provider = create_provider(mithril);

        this.store = createStore(
            this.provider.reducer,
            // nw.require('./common/mock.js'),
            applyMiddleware(
                send_filesystem(filesystem_client),
                ready_state_promise,
                Middleware.render(mithril, this.provider, window.document.body),
                ready_state_redraw(mithril)
            )
        );

        const action_adapter = new ActionCreatorAdapter(this.store);
        filesystem_client.on(
            'in_message',
            (message) => action_adapter.dispatch(message)
        );

        nw.require('./layout')(this.provider);
        nw.require('./entry')(this.provider);
        nw.require('./error_set')(this.provider);
    }

    run(){
        this.store.dispatch(ActionCreator.switch());
    }
};
