#!/bin/bash

hash rustfmt &>/dev/null || {
    echo "could not find 'rustfmt' within PATH" >&2;
    echo "please insstall 'rustfmt' within your PATH variable" >&2;
    exit 1;
};

find . -regex ".*\.rs$" -exec rustfmt {} \;
