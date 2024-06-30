#!/bin/bash

# A POSIX variable
OPTIND=1         # Reset in case getopts has been used previously in the shell.

# Initialize our own variables:
action=""
verbose=0

while getopts "h?va:" opt; do
  case "$opt" in
    h|\?)
      show_help
      exit 0
      ;;
    v)  verbose=1
      ;;
    a)  action=$OPTARG
      ;;
  esac
done

shift $((OPTIND-1))

[ "${1:-}" = "--" ] && shift

echo "verbose=$verbose, action='$action', Leftovers: $@"

if [ "$action" == "" ]; then
  echo "action is empty"
  exit 1
fi

if [ "$action" == "add" ]; then
  echo "add"
  sfctl cluster select
  sfctl application upload --path build/rcstore_root
  sfctl application provision --application-type-build-path rcstore_root
  sfctl application create --app-name fabric:/RcStore --app-type RcStore --app-version 0.0.1
elif [ "$action" == "remove" ]; then
  echo "remove"
  sfctl cluster select
  sfctl application delete --application-id RcStore
  sfctl application unprovision --application-type-name RcStore --application-type-version  0.0.1
else
  echo "unknown command"
  exit 1
fi