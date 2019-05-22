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

module.exports = {
    'id': 'jvs2qy94',
    'resource': 'Application',
    'consumer_state': {},
    'containers': [
        {
            'id': 'jsvs2qz18',
            'resource': 'Entry',
            'consumer_state': {
                'name': 'test_dir',
                'is_dir': true,
                'is_file': false
            },
            'containers': [
                {
                    'id': 'jsvs2qz20',
                    'resource': 'Entry',
                    'consumer_state': {
                        'name': 'test_nested_dir',
                        'is_dir': true,
                        'is_file': false
                    },
                    'containers': []
                },
                {
                    'id': 'jsvs2qz19',
                    'resource': 'Entry',
                    'consumer_state': {
                        'name': 'test_file',
                        'is_dir': false,
                        'is_file': true
                    },
                    'containers': []
                }
            ]
        },
    ]
};
