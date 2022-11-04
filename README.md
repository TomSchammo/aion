# aion

[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)

Aion is a command line tool to set timers.

It uses [notify-rust](https://github.com/hoodie/notify-rust) to send a dbus notification when
the timer is over.
Unfortunately sound [doesn't appear to work](https://github.com/hoodie/notify-rust/issues/58),
so this is a visual notification only.
