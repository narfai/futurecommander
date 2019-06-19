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

const { Icon } = nw.require('./common/icon');


module.exports = {
    'oninit': function(){
        this.context_menu = new nw.Menu();

        this.context_menu.append(new nw.MenuItem({ 'label': 'Copy there'}));

        this.show_menu = (event) => {
            event.preventDefault();
            this.context_menu.popup(event.x, event.y);
            return false;
        };

        this.spoil = () => {
            if(typeof this.action.list === 'undefined') throw new Error('Entry needs list action');
            this.action.list({ 'path': this.store.getState().cwd });
        };

        this.toggle_select = () => {
            if(this.store.getState().is_selected){
                this.action.unselect();
            } else {
                this.action.select();
            }
        };

        if(this.store.getState().is_open){
            this.spoil();
        }
    },
    'view': ({ state: { AnchorGroup, action, spoil, toggle_select, directory_create, show_menu, file_create, store_state: { is_open, name, is_dir, is_file, is_virtual, is_selected } }}) => {
        return m('div', [
            m('span',
                [
                    // Arrow
                    is_dir
                        ? is_open
                            ? m(
                                'span',
                                {onclick: action.close},
                                [Icon.angle_down()]
                            )
                            : m(
                                'span',
                                {onclick: spoil},
                                [Icon.angle_right()]
                            )
                        : Icon.empty(15, 15),
                    m(
                        is_selected ? 'span.selected' : 'span',
                        {onclick: toggle_select, contextmenu: show_menu},
                        [
                            // Icon
                            is_virtual
                                ? is_dir
                                ? is_open
                                    ? Icon.virtual_folder_open()
                                    : Icon.virtual_folder()
                                : is_file
                                    ? Icon.virtual_file()
                                    : '?'
                                : is_dir
                                ? is_open
                                    ? Icon.folder_open()
                                    : Icon.folder()
                                : is_file
                                    ? Icon.file()
                                    : '?',
                            // Entry name
                            name,
                        ]
                    ),
                    // Left buttons
                    is_dir
                        ? [
                            m(
                                'span',
                                {onclick: () => action.directory_create({ 'name': 'New directory' })},
                                [Icon.plus_directory()]
                            ),
                            m(
                                'span',
                                {onclick: () => action.file_create({ 'name': 'New file' })},
                                [Icon.plus_file()]
                            ),
                            is_selected
                                ? ''
                                : m(
                                    'span',
                                    {onclick: action.copy_there},
                                    [Icon.circle_down()]
                                )
                        ]
                        : '',
                    m(
                        'span',
                        {onclick: action.remove},
                        [Icon.times()]
                    )
                ]
            ),
            // Children
            is_dir && is_open
                ? m('ul', m(AnchorGroup, {'wrapper': 'li'}))
                : m('#')
        ])
    }
};
