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

module.exports = {
    Icon
};
