

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

const { Identity, Functional, ActionCreator } = nw.require('openmew-renderer');

const is_layout = ({ resource }) => resource === 'Layout';
const is_error_set = ({ resource }) => resource === 'ErrorSet';
const is_entry = ({ resource }) => resource === 'Entry';

const add_error_set_transducer = Identity.state_reducer(
    (next, state = null, action = {}) =>
        ((next_state) => (
                action.type === 'MESSAGE_ERROR'
                    && is_layout(next_state)
                    && typeof next_state.children.find(is_error_set) === 'undefined'
                        ? {
                            ...next_state,
                            'children': [
                                ...next_state.children,
                                Identity.module('ErrorSet', { 'errors': [action.message] })
                            ]
                        }
                        : next_state
            )
        )(next(state, action))
);

const osenv = require('osenv');
const { basename } = require('path');

const add_entry_transducer = Identity.state_reducer(
    (next, state = null, action = {}) =>
        ((next_state) => (
                action.type === ActionCreator.SWITCH_VIEWSET
                && is_layout(next_state)
                && typeof next_state.children.find(is_entry) === 'undefined'
                    ? {
                        ...next_state,
                        'children': [
                            ...next_state.children,
                            Identity.module(
                                'Entry',
                                {
                                    'name': basename(osenv.home()),
                                    'cwd': osenv.home(),
                                    'is_dir': true,
                                    'is_file': false,
                                    'is_open': true,
                                    'selected': false
                                }
                            )
                        ]
                    }
                    : next_state
            )
        )(next(state, action))
);

const add_to_selection_transducer = Identity.state_reducer(
    (next, state = null, action = {}) =>
        ((next_state) => (
                action.type === 'SELECT'
                && is_layout(next_state)
                && ( typeof next_state.selection === 'undefined'
                        || typeof next_state.selection.find(action.path) === 'undefined' )
                    ? {
                        ...next_state,
                        selection: [
                            action.path
                        ]
                    }
                    : next_state
            )
        )(next(state, action))
);

module.exports = Functional.pipe(
    add_error_set_transducer,
    add_entry_transducer,
    add_to_selection_transducer
);
