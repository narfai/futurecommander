

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

const { Identity, Functional } = nw.require('openmew-renderer');

const is_error_set = ({ resource }) => resource === 'ErrorSet';

const add_error = (state = [], action) => {
    return [
        ...state,
        action.message
    ];
};

const message_error_transducer = Identity.state_reducer(
    (next, state = null, action = {}) =>
        ((next_state) => (
                action.type === 'MESSAGE_ERROR'
                    && is_error_set(next_state)
                        ? {
                            ...next_state,
                            'errors': add_error(next_state.errors, action)
                        }
                        : next_state
            )
        )(next(state, action))
);

module.exports = Functional.pipe(
    message_error_transducer
);
