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

const { FileSystemClient }  = require('../app_node/filesystem/client');
const { filesystem_thunk, readyStatePromise } = require('./middleware.js');

const m = nw.require('mithril');
const {
    Registry,
    register_middleware,
    attach_middleware,
    detach_middleware,
    Action
} = nw.require('openmew-renderer');


const {
    createStore,
    applyMiddleware
} = nw.require('redux');


const filesystem_client = new FileSystemClient();
const registry = new Registry();
const store = createStore(
    (state) => state,
    {},//nw.require('app_web/mock'),
    applyMiddleware(
        register_middleware(registry),
        attach_middleware(registry),
        detach_middleware(registry),
        // readyStatePromise,
        filesystem_thunk(filesystem_client)
    )
);

const ApplicationBlueprint = nw.require('./application');
const EntryBlueprint = nw.require('./entry');

store.dispatch(Action.REGISTER_BLUEPRINT(ApplicationBlueprint));
store.dispatch(Action.REGISTER_BLUEPRINT(EntryBlueprint));

store.dispatch(Action.ATTACH({
    'resource': ApplicationBlueprint.resource,
    'render': ({ container }) => {
        store.replaceReducer(container.reducer);
        m.mount(document.body, container.component);
        store.subscribe(() => {
            console.log('state change !', store.getState());
            m.redraw();
        });
    }
}));

console.log('WEB MAIN');
