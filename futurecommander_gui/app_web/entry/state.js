

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

const path = require('path');

const { Identity, Functional } = nw.require('openmew-renderer');

const is_entry = ({ resource }) => resource === 'Entry';
const has_name = (target_name) => ({ name }) => name === target_name;

const list_entry = (state, action) => {
    const entry_collection = action.payload.entries;

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
                    cwd: path.join(action.payload.path, entry.name),
                    is_open: false
                }
            )
        );

    return [
        ...entry_children,
        ...to_add
    ].sort(
        (left, right) =>
            right.is_dir - left.is_dir
                || left.name.localeCompare(
                    right.name,
                    undefined,
                    { numeric: true }
                )
    );
};

const list_entry_transducer = Identity.state_reducer(
    (next, state = null, action = {}) =>
        ((next_state) => (
                action.type === 'DirectoryRead'
                // && typeof action.ready !== 'undefined'
                // && action.ready === true
                && action.payload.path === state.cwd
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

module.exports = Functional.pipe(
    list_entry_transducer,
    close_entry_transducer
);
