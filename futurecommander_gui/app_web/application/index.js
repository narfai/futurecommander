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
const { AnchorGroup, Spread, Action } = nw.require('openmew-renderer');
const osenv = require('osenv');
const { basename } = require('path');
//TODO depends on initial path or user input to have full path ?
//TODO renderer's internal_middleware ?
//TODO there is a ordered functional pipe to find

module.exports = {
    'resource': 'Application',
    'controller': function ApplicationController({ container }){
        const { containers } = container.store.getState();
        this.open = (path) => (Spread.self_scope(container, () => Action.ATTACH({
                'resource': 'Entry',
                'parent_id': container.id,
                'consumer_state': {
                    'cwd': path,
                    'name': basename(path),
                    'is_file': false,
                    'is_dir': true,
                    'open': true
                },
                'render': ({ container }) => {
                    console.log('render of submodule', container);
                }
            })
        )());

        if(containers.length === 0) {
            const home_directory = osenv.home();
            this.open(home_directory);
        }
    },
    'view': ({ registry, container }) => () => m(
        'div',
        [
            m('h1', 'Hello World'),
            m('ul',
                [
                    m(AnchorGroup, {
                        id: container.id,
                        registry,
                        wrapper: 'li'
                    })
                ]
            )
        ]
    )
    // 'reducer': (state) => state
};
