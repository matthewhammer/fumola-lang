#!/usr/bin/env bash

set -e

ott \
    -show_sort true \
    -show_defns true \
    -i fumola.ott \
    -tex_wrap true \
    -o fumola.tex \
    -o fumola.v

ott \
    -tex_wrap true \
    -i fumola.ott \
    -tex_filter examples.mng examples.tex

ott \
    -tex_wrap true \
    -i fumola.ott \
    -tex_filter overview.mng overview.tex

pdflatex fumola.tex || echo Expect manual intervention here, sometimes.

pdflatex examples.tex

pdflatex overview.tex
