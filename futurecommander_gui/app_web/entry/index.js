/*
 * Copyright 1519 François CADEILLAN
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
const { Request } = require('../../app_node/filesystem/request');
const path = require('path');

class Icon {
    static icon(width, height, path) {
        return m('img', {
            'height': height + 'px',
            'width': width + 'px',
            'src': '/node_modules/@fortawesome/fontawesome-free/svgs/' + path
        });
    }

    static empty(height, width){
        return m('img', {
            'height': height + 'px',
            'width': width + 'px'
        });
    }

    static folder_15() {
        return Icon.icon(15, 15, 'solid/folder.svg')
    }

    static file_15() {
        return Icon.icon(15, 15, 'solid/file.svg')
    }

    static angle_right_15() {
        return Icon.icon(15, 15, 'solid/angle-right.svg')
    }

    static angle_down_15() {
        return Icon.icon(15, 15, 'solid/angle-down.svg')
    }
}

const osenv = require('osenv');

module.exports = {
    'resource': 'Entry',
    'reducer': (state = { 'cwd': osenv.home(), 'name': '', 'is_dir': false, 'is_file': false, 'open': false }, action) => {
        switch (action.type) {
            case 'OPEN_DIRECTORY':
                return {
                    ...state,
                    open: true
                };
            case 'CLOSE_DIRECTORY':
                return {
                    ...state,
                    open: false
                };
            default:
                return state;
        }
    },
    'controller': function EntryController({ container, registry }) {
        this.open = (e) => {
            Spread.self_scope(container, () => ({ 'type': 'OPEN_DIRECTORY' }))();
            this.refresh();
            return e;
        };

        this.close = (e) => {
            const { containers } = container.store.getState();
            Spread.self_scope(container, () => ({ 'type': 'CLOSE_DIRECTORY' }))();
            containers.forEach(({ id }) =>
                Spread.self_scope(
                    container,
                    () => Action.DETACH({ id })
                )()
            );
        };

        this.refresh = () => {
            const { consumer_state: { open, cwd, is_dir }, containers} = container.store.getState();
            if(open && is_dir) {
                return container.store.dispatch(
                    (dispatch, filesystem_client) =>
                        filesystem_client
                            .send(Request.list({'path': cwd}))
                            .then((response) => {
                                if(response.is_success()){
                                    const entry_collection = response.result();
                                    entry_collection.sort((left, right) => right.is_dir - left.is_dir);

                                    entry_collection
                                        .filter(({ name }) =>
                                            !containers
                                                .find(({ resource, consumer_state}) =>
                                                    resource === 'Entry'
                                                    && consumer_state.name === name
                                                )
                                        )
                                        .forEach(
                                            (entry) =>
                                                Spread.custom_scope(
                                                    container,
                                                    () => Action.ATTACH({
                                                        'resource': 'Entry',
                                                        'parent_id': container.id,
                                                        'consumer_state': {
                                                            'cwd': path.join(cwd, entry.name),
                                                            'name': entry.name,
                                                            'is_file': entry.is_file,
                                                            'is_dir': entry.is_dir,
                                                            'open': false
                                                        },
                                                        'render': ({container}) => {
                                                            console.log('render of submodule', container);
                                                        }
                                                    }),
                                                    ({ resource, consumer_state = {}, ...other}) =>
                                                        resource === 'Entry'
                                                        && typeof consumer_state.cwd !== 'undefined'
                                                        && consumer_state.cwd === cwd
                                                )()
                                        );

                                    containers
                                        .filter(({ resource, consumer_state: { name } }) =>
                                            resource === 'Entry'
                                            && entry_collection.find((entry) => entry.name === name)
                                        )
                                        .forEach(({ id }) => Spread.self_scope(
                                            container,
                                            () => Action.DETACH({ id })
                                        )());
                                } else {
                                    console.error(response.result());
                                }
                            })
                );
            }
        };
        this.refresh();
    },
    'view': ({ container, registry }) => (vnode) => {
        const { name = '', is_dir = false, is_file = false, open = false } = container.consumer_state();
        return m(//TODO button refresh
            '#',
            [
                is_dir
                    ? open
                        ? m(
                            'span',
                            {
                                onclick: vnode.state.close
                            },
                            [ Icon.angle_down_15() ]
                        )
                        : m(
                            'span',
                            {
                                onclick: vnode.state.open
                            },
                            [ Icon.angle_right_15() ]
                        )
                    : Icon.empty(15, 15),
                is_dir
                    ? Icon.folder_15()
                    : is_file
                        ? Icon.file_15()
                        : '?',
                m('span', ' ' + name),
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
        );
    }
    // 'reducer': (state) => state
};
