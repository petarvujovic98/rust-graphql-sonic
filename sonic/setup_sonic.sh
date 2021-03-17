#!/bin/sh

if [ $EUID -ne 0 ]; then
  exec sudo $0
fi

BASEDIR="$(cd "$(dirname $0)" && pwd)"

if [ -f /etc/sonic.cfg ]; then
  rm -f /etc/sonic.cfg
fi

ln -s "$BASEDIR/sonic.cfg" /etc/sonic.cfg

if [ -f /etc/systemd/system/sonic.service ]; then
  rm -f /etc/systemd/system/sonic.service
fi

ln -s "$BASEDIR/sonic.service" /etc/systemd/system/sonic.service

systemctl start sonic.service
