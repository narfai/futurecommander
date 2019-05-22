
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

const readyStatePromise = store => next => action => {
    if (!action.promise) {
        return next(action);
    }

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

const filesystem_thunk = (filesystem_client) => (store) => (next) => (action) =>
    typeof action === 'function'
        ? action(store.dispatch, filesystem_client)
        : next(action);

module.exports = {
    filesystem_thunk,
    readyStatePromise
};
