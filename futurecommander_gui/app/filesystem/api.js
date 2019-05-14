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


// module.exports.List = class {
//     construct(path) {
//         this.id = null;
//         this.path = path;
//         this.type = 'LIST';
//     }
// };

const uniqid = require('uniqid');

class Entry {
    constructor({ name = null, is_dir = null, is_file = null }) {
        this.name = name;
        this.is_dir = is_dir;
        this.is_file = is_file;
    }
}

class Response {
    constructor({ id = null, status = null, kind = null, error = null, content = [] }) {
        this.id = id;
        this.status = status;
        this.kind = kind;
        this.error = error;
        this.content = content
            .map((entry) => new Entry(entry));
    }

    result() {
        switch(this.status) {
            case 'Success':
                switch(this.kind) {
                    case 'Collection':
                        return this.content;
                    case 'Entry':
                        if (this.content.length > 0) {
                            return this.content[0]
                        }
                        return null;
                }
                break;
            case 'Fail':
                throw new Error(this.error);
        }
    }
}

class Request {
    constructor({ id, type, ...parameters}) {
        this.id = id;
        this.type = type;
        this.parameters = parameters;
    }

    get_id() {
        return this.id;
    }

    get_type() {
        return this.type;
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

    static list(path) {
        return {
            id: Request.strong_id(),
            type: 'LIST',
            path: path
        }
    }
}

module.exports = {
    Entry,
    Response,
    Request
};
