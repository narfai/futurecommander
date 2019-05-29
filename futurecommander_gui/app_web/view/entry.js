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

class Icon {
    static icon(width, height, path) {
        return m('img', {
            'height': height + 'px',
            'width': width + 'px',
            'src': '/node_modules/@fortawesome/fontawesome-free/svgs/' + path
        });
    }

    static empty(height, width){
        return m('img', {
            'height': height + 'px',
            'width': width + 'px'
        });
    }

    static folder_15() {
        return Icon.icon(15, 15, 'solid/folder.svg')
    }

    static file_15() {
        return Icon.icon(15, 15, 'solid/file.svg')
    }

    static angle_right_15() {
        return Icon.icon(15, 15, 'solid/angle-right.svg')
    }

    static angle_down_15() {
        return Icon.icon(15, 15, 'solid/angle-down.svg')
    }
}

const m = nw.require('mithril');

module.exports = {
    'oninit': function(){
        this.controls = {
            spoil: (event) => {
                if(typeof this.action.list === 'undefined') throw new Error('Entry needs list action');

                const { cwd } = this.store.getState();
                console.log('CWD', cwd);
                return this.action.list({ 'path': cwd }).result
                    .then((response) => {
                        console.log('response from view', response);
                        m.redraw();
                        return event;
                    });
            },
            unspoil: (event) => {
                console.log('unspoil', event, this.is_open);
                this.action.close();
                // event.redraw = true;
                // m.redraw();
                return event;
            }
        };

        if(this.store.getState().is_open){
            this.controls.spoil();
        }
    },
    'view': ({ state: { AnchorGroup, action, controls, store_state: { is_open, name, is_dir, is_file } }}) => {
        return m('div', [
            m('span',
                [
                    //Arrow
                    is_dir
                        ? is_open
                            ? m(
                                'span',
                                // @NOTICE prevent from unfilled action during development
                                {onclick: controls.unspoil},
                                [Icon.angle_down_15()]
                            )
                            : m(
                                'span',
                                {onclick: controls.spoil},
                                [Icon.angle_right_15()]
                            )
                        : Icon.empty(15, 15),
                    //Icon
                    is_dir
                        ? Icon.folder_15()
                        : is_file
                            ? Icon.file_15()
                            : '?',
                    name
                ]
            ),
            is_dir && is_open
                ? m('ul', m(AnchorGroup, {'wrapper': 'li'}))
                : m('#')
        ])
    }
};
