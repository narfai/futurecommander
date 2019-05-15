
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

class ExplorerComponent {
    constructor({ 'attrs': { filesystem_client } }){
        // this.persister = persister;
        // this.provider = provider;
        // this.actions = actions;
        // this.getState = this.provider.provide(
        //     ({ 'contact': { name, title }, 'application': { locked, modifiedAt }}) => ({ locked, name, title, modifiedAt })
        // );

        this.on_main_click = () => {
            filesystem_client.send(filesystem_client.Request.list({ path: '/home/narfai/tmp2' }))
                .then((response) => {
                    console.log(response);
                });
        };
    }

    view({ attrs }){
        console.log('APP VIEW', attrs);
        // let { locked, name, title } = this.getState();

        return (m("h1", {
            onclick: () => this.on_main_click()
        }, "Hello web context inside node"));
    }
}

module.exports = {
    ExplorerComponent
};
