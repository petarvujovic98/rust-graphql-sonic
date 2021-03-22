#!/bin/sh

BASEDIR="$(cd "$(dirname $0)" && pwd)"

cd $BASEDIR && git clone git@github.com:JeffSackmann/tennis_atp.git
