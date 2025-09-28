#!/usr/bin/env bash
# shellcheck disable=SC2004,SC2206,SC2068,SC2086
set -e
declare -a cases=(
    "kebab"
    "camel"
    "pascal"
    "shouty-kebab"
    "shouty-snake"
    "snake"
    "train"
)
h3_examples() {
for case_type in ${cases[@]}; do
    cat <<EOF
### $(backtick)heck-string --to=${case_type}$(backtick)

Convert string to ${case_type} case

$(backtick)$(backtick)$(backtick)shell
$(make_variants "${case_type}")
$(backtick)$(backtick)$(backtick)

EOF
done
}
enum_variants() {
    for case_type in ${cases[@]}; do
        echo " - $(wrap_backtick "${case_type}")"
    done
}
make_variants() {
    to="$1"
    if [ -z "${to}" ]; then
        1>&2 echo "[make_variants error] missing argument"
        exit 101
    fi
    base_red=249
    base_green=39
    base_blue=114
    for index in ${!cases[@]}; do
        case_type=${cases[$index]}
        if [ "${case_type}" == "${to}" ]; then
            continue
        fi
        delta=$(( $index + 2 ))
        if [ $delta -gt 0 ]; then
            if [ $(( $delta % 3 )) -eq 0 ]; then
                red=$(( ($base_red + ($delta * 25)) % 255 ))
            elif [ $(( $delta % 2 )) -eq 0 ]; then
                green=$(( ($base_green + ($delta * 25)) % 255 ))
            else
                blue=$(( ($base_blue + ($delta * 25)) % 255 ))
            fi
        fi
        echo "\$ heck-string --to=${to} \"$(heck-string --to=${case_type} "example to ${case_type} case")\""
        heck-string --to=${to} "$(heck-string --to=${case_type} "example to ${to} case")"
        echo
    done
}
backtick(){
    python -c 'print(chr(96))'
}
wrap_backtick(){
    if [ -n "$@" ]; then
        echo -n "$(backtick)${@}$(backtick)"
    fi
}

readme() {

cat <<EOF
# heck-string-cli

command-line tool to apply [heck](https://crates.io/crates/heck) convert case of strings $(wrap_backtick "--to"):
$(enum_variants)

## Installation

$(backtick)$(backtick)$(backtick)shell
cargo install heck-string-cli
$(backtick)$(backtick)$(backtick)

## Usage

$(h3_examples)

EOF
}

if [ "$0" == "${BASH_SOURCE[0]}" ]; then
    readme > README.md
    1>&2 echo "generated: README.md"
fi
