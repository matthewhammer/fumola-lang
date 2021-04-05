#!/usr/bin/env bash

set -e

cat fumola.ott scratch.ott > fumola-scratch.ott

ott \
    -show_sort true \
    -show_defns true \
    -tex_wrap true \
    -i fumola-scratch.ott \
    -o fumola.tex \

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
