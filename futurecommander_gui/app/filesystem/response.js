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

module.exports = {
    Entry,
    Response
};
