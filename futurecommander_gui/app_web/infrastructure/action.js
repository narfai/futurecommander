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

const { ActionTransducer, Functional } = nw.require('openmew-renderer');

class InfraScope {
    static wrapper_or_parent({ resource, cwd = null }, { "payload": { path }}){
        return resource === 'Layout' || (resource === 'Entry' && path.includes(cwd));
    }

    static same_entry({ resource, cwd = null }, { 'payload': { path } }) {
        return resource === 'Entry' && cwd === path;
    }

    static entry_scope(/*store*/){
        return Functional.pipe(
            ActionTransducer.propagate(InfraScope.wrapper_or_parent),
            ActionTransducer.reduce(InfraScope.same_entry)
        );
    }
}

module.exports = (spread) => ({
    'DirectoryRead': spread(
        ({ 'payload': { entries, path }}) => ({
            'type': 'DIRECTORY_READ',
            entries,
            path
        })
    )(InfraScope.entry_scope, spread.redraw.allow)
});
