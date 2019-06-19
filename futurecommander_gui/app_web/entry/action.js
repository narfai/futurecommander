
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

const path = require('path');

module.exports = (spread) => ({
        'list': spread(
            ({state, event: { path }}) => ({
                'type': 'DIRECTORY_OPEN',
                'filesystem_header': 'DirectoryOpen',
                'payload': {
                    path
                }
            })
        )(spread.scope.self),
        'directory_create': spread(
            ({state, event: { name }}) => ({
                'type': 'DIRECTORY_CREATE',
                'filesystem_header': 'DirectoryCreate',
                'payload': {
                    path: path.join(state.cwd, name),
                    overwrite: false,
                    recursive: false
                }
            })
        )(spread.scope.self),
        'file_create': spread(
            ({state, event: { name }}) => ({
                'type': 'FILE_CREATE',
                'filesystem_header': 'FileCreate',
                'payload': {
                    path: path.join(state.cwd, name),
                    overwrite: false,
                    recursive: false
                }
            })
        )(spread.scope.self),
        'remove': spread(
            ({ state }) => ({
                'type': 'REMOVE',
                'filesystem_header': 'Remove',
                'payload': {
                    'path': state.cwd,
                    recursive: true
                }
            })
        )(spread.scope.self),
        'close': spread(
            () => ({
                'type': 'CLOSE'
            })
        )(spread.scope.self, spread.redraw.allow)
});
