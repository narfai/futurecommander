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

const { Renderer, Functional } = nw.require('openmew-renderer');
const { entry, entry_container } = nw.require('./behavior');
const LAYOUT = 'Layout';
const ENTRY = 'Entry';

module.exports = {
    LAYOUT,
    ENTRY,
    'connect': (provider) => {
        provider.connect_component_transducers(
            Functional.pipe( //Order matter
                // @NOTICE this transducer provide `vnode.state.store_state` to component's view
                // Renderer.debug_redraw(Renderer.state_aware),
                Renderer.state_aware,
                // @NOTICE this transducer optimize by preventing mithril to evaluate redraws of unchanged components
                // according to the state tree ( depends on state_aware for its diff )
                Renderer.skip_redraw,
            )
        );
        provider.connect_component(LAYOUT, nw.require('./view/layout'), entry_container);
        provider.connect_component(ENTRY, nw.require('./view/entry'), entry);
    }
};
