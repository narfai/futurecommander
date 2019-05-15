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

const mithril               = nw.require('mithril');
const { Application }       = nw.require('./application.js');

const { FileSystemClient }  = require('../app_node/filesystem/client');
const { Provider }          = require('../app_node/store/provider');
const { Action }            = require('../app_node/store/action');

const preload    = {}; //TODO stub
const reducers   = {};
const middleware = {};

const provider          = new Provider({ preload, reducers, middleware });
const action            = new Action(provider.store);

const filesystem_client = new FileSystemClient();

mithril.mount(document.body, () => new Application({
        'attrs': {
            filesystem_client,
            provider,
            action
        }
    })
);

console.log('WEB MAIN');
