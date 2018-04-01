#!/bin/sh

install () {

set -eu

UNAME=$(uname)
if [ "$UNAME" != "Linux" -a "$UNAME" != "Darwin" -a "$UNAME" != "OpenBSD" ] ; then
    echo "Sorry, OS not supported: ${UNAME}. Download binary from https://github.com/getapp/app/releases"
    exit 1
fi

if [ "$UNAME" = "Darwin" ] ; then
  OSX_ARCH=$(uname -m)
  if [ "${OSX_ARCH}" = "x86_64" ] ; then
    PLATFORM="darwin_amd64"
  else
    echo "Sorry, architecture not supported: ${OSX_ARCH}. Download binary from https://github.com/getapp/app/releases"
    exit 1
  fi
elif [ "$UNAME" = "Linux" ] ; then
  LINUX_ARCH=$(uname -m)
  if [ "${LINUX_ARCH}" = "i686" ] ; then
    PLATFORM="linux_386"
  elif [ "${LINUX_ARCH}" = "x86_64" ] ; then
    PLATFORM="linux_amd64"
  else
    echo "Sorry, architecture not supported: ${LINUX_ARCH}. Download binary from https://github.com/getapp/app/releases"
    exit 1
  fi
elif [ "$UNAME" = "OpenBSD" ] ; then
    OPENBSD_ARCH=$(uname -m)
  if [ "${OPENBSD_ARCH}" = "amd64" ] ; then
      PLATFORM="openbsd_amd64"
  else
      echo "Sorry, architecture not supported: ${OPENBSD_ARCH}. Download binary from https://github.com/getapp/app/releases"
      exit 1
  fi

fi

LATEST=$(curl -s https://api.github.com/repos/getapp/app/tags | grep -Eo '"name":.*?[^\\]",'  | head -n 1 | sed 's/[," ]//g' | cut -d ':' -f 2)
URL="https://github.com/getapp/app/releases/download/$LATEST/appa_$PLATFORM"
DEST=${DEST:-/usr/local/bin/appa}

if [ -z $LATEST ] ; then
  echo "Error requesting. Download binary from https://github.com/getapp/app/releases"
  exit 1
else
  curl -sL https://github.com/getapp/app/releases/download/$LATEST/appa_$PLATFORM -o $DEST
  chmod +x $DEST
fi
}

install