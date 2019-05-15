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

const { createStore, combineReducers, applyMiddleware } = require('redux');

const logger = (store) => (next) => (action) => {
    /* eslint-disable no-console */
    console.group(action.type);
    console.info('dispatching', action);
    let result = next(action);
    console.log('next state', store.getState());
    console.groupEnd();
    return result;
    /* eslint-enable no-console */
};


export class Provider {
    constructor({ preload, reducers = {}, middleware = []}){
        this.store = createStore(
            combineReducers(reducers),
            preload,
            applyMiddleware([logger, ...middleware])
        );
    }

    provide(select){
        return () => select(this.getState());
    }

    getState(){
        return this.store.getState();
    }
}

module.exports = {
    Provider
};
