
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
const ready_state_promise = (store) => (next) => (action) => {
    if (!action.promise) return next(action);

    function makeAction(ready, data) {
        const newAction = Object.assign({}, action, { ready }, data);
        delete newAction.promise;
        return newAction;
    }

    next(makeAction(false));
    return action.promise.then(
        result => next(makeAction(true, { result })),
        error => next(makeAction(true, { error }))
    )
};

const thunk = (store) => (next) => (action) =>
    typeof action === 'function'
        ? action(store.dispatch, store.getState)
        : next(action);

const list_filesystem = (filesystem_client) => (/*redux_store*/) => (next) => (action) => {
    if(action.type !== 'DIRECTORY_OPEN') return next(action);

    const next_action = next(action);
    filesystem_client.emit(
        'out_message',
        filesystem_client.message({
            'header': 'DirectoryOpen',
            'payload': {
                'path': action.path
            }
        })
    );
    return next_action;
};

const ready_state_redraw = (mithril) => (/*redux_store*/) => (next) => (action) => {
    const result = next(action);
    if(
        typeof action.redraw !== 'undefined'
        && action.redraw
        && (typeof action.ready === 'undefined' || action.ready)
    ) mithril.redraw.sync();
    return result;
};
//TODO error handling middleware


module.exports = {
    list_filesystem,
    thunk,
    ready_state_promise,
    ready_state_redraw
};
