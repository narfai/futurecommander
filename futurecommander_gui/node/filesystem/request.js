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

const uniqid = require('uniqid');

class Request {
    constructor({ type, ...parameters }) {
        this.cursor = 0;
        this.keys = Object.keys(parameters);
        this.type = type;
        this.parameters = parameters;
    }

    get_type() {
        return this.type;
    }

    next_key() {
        if (typeof this.keys[this.cursor] !== undefined) {
            let key = this.keys[this.cursor];
            this.cursor++;
            return key;
        }
        return null;
    }

    get_parameter(key) {
        return this.parameters.hasOwnProperty(key)
            ? this.parameters[key]
            : null;
    }

    static from({ id, type, ...obj}) {
        let request = new Request(id, type);
        switch (type) {
            case 'LIST':
                return ListRequest.apply(request, obj);
            default:
                throw new Error(`Unknown request ${type}`)
        }
    }

    static strong_id() {
        return Array.from(uniqid.process())
            .reduce(
                (acc, cur) => acc + cur.charCodeAt(0).toString(10),
                '0'
            )
    }

    static list({ path }) {
        return {
            id: Request.strong_id(),
            type: 'LIST',
            path
        }
    }

    static status({ path} ) {
        return {
            id: Request.strong_id(),
            type: 'STATUS',
            path
        }
    }

    static reset() {
        return {
            id: Request.strong_id(),
            type: 'RESET'
        }
    }

    static apply() {
        return {
            id: Request.strong_id(),
            type: 'APPLY'
        }
    }

    static copy({ source, destination, merge = false, overwrite = false }) {
        return {
            id: Request.strong_id(),
            type: 'COPY',
            source,
            destination,
            merge,
            overwrite
        }
    }

    static move({ source, destination, merge = false, overwrite = false }) {
        return {
            id: Request.strong_id(),
            type: 'MOVE',
            source,
            destination,
            merge,
            overwrite
        }
    }

    static create({ path, kind, recursive = false, overwrite = false }) {
        return {
            id: Request.strong_id(),
            type: 'CREATE',
            kind,
            recursive,
            overwrite
        }
    }

    static delete({ path, recursive = false }) {
        return {
            id: Request.strong_id(),
            type: 'DELETE',
            path,
            recursive
        }
    }

    static save({ path, overwrite = false }) {
        return {
            id: Request.strong_id(),
            type: 'SAVE',
            path,
            overwrite
        }
    }

    static import({ path }) {
        return {
            id: Request.strong_id(),
            type: 'IMPORT',
            path
        }
    }
}

module.exports = {
    Request
};
