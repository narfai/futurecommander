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

const m = nw.require('mithril');

class CleanupErrorsTimeout {
    constructor(){
        this.index = 0;
        this.timeout = null;
    }
    next(){
        this.timeout = setTimeout(() => {
            clearTimeout(this.timeout);
            this.index++;
            m.redraw.sync();
        }, 3000);
    }
}

module.exports = {
    'oninit': function(){
        this.timeout = new CleanupErrorsTimeout();
        this.timeout.next();
    },
    'onupdate': function(){
        if(this.timeout.index < this.store_state.errors.length){
            this.timeout.next()
        }
    },
    'view': ({ state: { timeout, AnchorGroup, action, store_state: { errors } }}) => {
        return m(
            'ul',
            errors.slice(timeout.index).map((message) => m('li', message))
        )
    }
};
