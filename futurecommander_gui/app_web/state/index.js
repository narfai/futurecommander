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

const { Utility, Structural, Middleware, Identity } = nw.require('openmew-renderer');
const { createStore, applyMiddleware } = nw.require('redux');
const { list_filesystem, ready_state_promise, ready_state_redraw } = nw.require('./state/middleware');
const mithril = nw.require('mithril');


const is_entry = ({ resource }) => resource === 'Entry';
const has_name = (target_name) => ({ name }) => name === target_name;

const path = require('path');

const list_entry = (state, action) => {
    const { ready = null, result = null } = action;
    if(!ready || result === null || state.cwd !== action.path) return state.children;

    const entry_collection = result.result();

    const entry_children = state.children
        .filter(is_entry)
        .filter(
            ({ name }) => entry_collection.find(has_name(name))
        );

    const to_add = entry_collection
        .filter(
            ({ name }) => !entry_children.find(has_name(name))
        ).map(
            (entry) => Identity.module(
                'Entry',
                {
                    ...entry,
                    cwd: path.join(action.path, entry.name),
                    is_open: false
                }
            )
        );

    return [
        ...entry_children,
        ...to_add
    ].sort((left, right) => right.is_dir - left.is_dir || left.name.localeCompare(right.name, undefined, {numeric: true}));
};

const list_entry_transducer = Identity.state_reducer(
    (next, state = null, action = {}) =>
        ((next_state) => (
                action.type === 'LIST'
                    ? {
                        ...next_state,
                        is_open: true,
                        'children': list_entry(next_state, action)
                    }
                    : next_state
            )
        )(next(state, action))
);


const close_entry_transducer = Identity.state_reducer(
    (next, state = null, action = {}) =>
        ((next_state) => (
                action.type === 'CLOSE'
                    ? {
                        ...next_state,
                        is_open: false
                    }
                    : next_state
            )
        )(next(state, action))
);

module.exports = {
    'connect': (window, provider, filesystem_client, mock) => {
        const { logger } = Utility;
        const { detach, append, prepend } = Structural;

        provider.connect_state_transducers(
            logger,
            detach,
            append,
            prepend,
            list_entry_transducer,
            close_entry_transducer
        );

        return createStore(
            provider.reducer,
            mock,
            applyMiddleware(
                list_filesystem(filesystem_client),
                ready_state_promise,
                Middleware.render(mithril, provider, window.document.body),
                ready_state_redraw(mithril),
                // Middleware.redraw(mithril)
            )
        )
    }
};
