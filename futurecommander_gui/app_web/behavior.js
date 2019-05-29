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

const osenv = require('osenv');
const { basename } = require('path');

module.exports = {
    entry: (spread) => ({
        'list': spread(
            ({state, event: { path }}) => ({
                'type': 'LIST',
                path
            })
        )(spread.scope.self, spread.redraw.allow),
        'close': spread(
            ({state, event: { path }}) => ({
                'type': 'CLOSE'
            })
        )(spread.scope.self, spread.redraw.allow)
    }),
    entry_container: (spread) => ({
        'entry': spread.append(({state, event}) => {
            const path = event.path === null ? osenv.home() : event.path;
            return {
                'resource': 'Entry',
                'initial_state': {
                    'name': basename(path),
                    'cwd': path,
                    'is_dir': true,
                    'is_file': false,
                    'is_open': true
                }
            };
        })(spread.scope.self, spread.redraw.allow)
    })
};
