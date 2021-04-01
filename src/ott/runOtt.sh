#!/usr/bin/env bash

ott \
    -show_sort true \
    -show_defns true \
    -i fumola.ott \
    -tex_wrap true \
    -o fumola.tex \
    -o fumola.v

ott \
    -i fumola.ott \
    -tex_filter examples.mng examples.tex
