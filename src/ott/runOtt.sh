#!/usr/bin/env bash

set -x

ott \
    -show_sort true \
    -show_defns true \
    -i fumola.ott \
    -tex_wrap true \
    -o fumola.tex \
    -o fumola.v

pdflatex fumola.tex

ott \
    -tex_wrap true \
    -i fumola.ott \
    -tex_filter examples.mng examples.tex

pdflatex examples.tex
