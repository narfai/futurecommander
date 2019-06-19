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
const osenv = require('osenv');

const is_error_set = ({ resource }) => resource === 'ErrorSet';

module.exports = {
    'oninit': function({ state: { store, action } }){
        // const { children = null } = store.getState();
        // if(children !== null && !(children.length > 0)){
        //     action.entry({ path: osenv.home() });
        // }
    },
    'view': ({ state: { AnchorGroup }}) =>
        m('#',
            m('h1', 'Layout'),
            m('nav', [
                m(AnchorGroup, { filterFn: (state) => is_error_set(state) })
            ]),
            m('main', m(AnchorGroup, { filterFn: (state) => !is_error_set(state) }))
        )
};
