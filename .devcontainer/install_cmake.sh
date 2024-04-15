#!/bin/bash

export CMAKE_VERSION="3.29.0-rc3"

wget -qO- "https://github.com/Kitware/CMake/releases/download/v$CMAKE_VERSION/cmake-$CMAKE_VERSION-linux-x86_64.tar.gz" | tar --strip-components=1 -xz -C /usr/local
