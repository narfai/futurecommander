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

const STATUS_FAIL = 'Fail';
const STATUS_SUCCESS = 'Success';

const RESULT_ENTRY = 'Entry';
const RESULT_COLLECTION = 'Entries';

class Entry {
    constructor({ name = null, is_dir = null, is_file = null, is_virtual = null }) {
        this.name = name;
        this.is_dir = is_dir;
        this.is_file = is_file;
        this.is_virtual = is_virtual;
    }
}

class Response {
    constructor({ id = null, status = null, header = null, body = null }) {
        this.id = id;
        this.header = header;
        this.status = status;
        this.body = body;
    }

    is_fail(){
        return this.status === STATUS_FAIL;
    }

    is_success(){
        return this.status === STATUS_SUCCESS
    }

    parse() {
        switch(this.status) {
            case STATUS_SUCCESS:
                switch(this.header) {
                    case RESULT_COLLECTION:
                        if(this.body !== null){
                            return this.body.map((entry) => new Entry(entry));
                        }
                        return null;
                }
                break;
            case STATUS_FAIL:
                throw new Error(this.body);
        }
    }
}

module.exports = {
    Entry,
    Response
};
