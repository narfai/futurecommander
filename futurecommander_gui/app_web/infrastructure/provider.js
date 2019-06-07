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

const { Provider, Utility, Structural, Renderer } = nw.require('openmew-renderer');

module.exports = function create_provider(mithril){
    const provider = new Provider(mithril, 'Layout');

    provider.connect_component_transducers(
        Renderer.state_aware,
        Renderer.skip_redraw
    );

    const { logger } = Utility;
    const { detach, append, prepend } = Structural;

    provider.connect_state_transducers(
        logger,
        detach,
        append,
        prepend
    );

    return provider;
};
