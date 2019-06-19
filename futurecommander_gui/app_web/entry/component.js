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
const path = require('path');

module.exports = {
    'oninit': function(){
        this.spoil = () => {
            if(typeof this.action.list === 'undefined') throw new Error('Entry needs list action');
            this.action.list({ 'path': this.store.getState().cwd });
        };

        if(this.store.getState().is_open){
            this.spoil();
        }
    },
    'view': ({ state: { AnchorGroup, action, spoil, directory_create, file_create, store_state: { is_open, name, is_dir, is_file, is_virtual } }}) => {
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
