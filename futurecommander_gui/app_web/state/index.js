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

const { Utility, Structural, Middleware } = nw.require('openmew-renderer');
const { createStore, applyMiddleware } = nw.require('redux');
const { list_filesystem, ready_state_promise } = nw.require('./state/middleware');
const mithril = nw.require('mithril');


module.exports = {
    'connect': (window, provider, filesystem_client, mock) => {
        const { logger } = Utility;
        const { detach, append, prepend } = Structural;

        provider.connect_state_transducers(
            logger,
            detach,
            append,
            prepend
        );

        return createStore(
            provider.reducer,
            mock,
            applyMiddleware(
                list_filesystem(filesystem_client),
                ready_state_promise,
                Middleware.render(mithril, provider, window.document.body),
                Middleware.redraw(mithril)
            )
        )
    }
};
