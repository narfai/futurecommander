#!/bin/bash

PROJECTPATH="$( cd "$(dirname "$0")/.." ; pwd -P )"
find ""${PROJECTPATH}"" -name '*.rs' | xargs wc -l
