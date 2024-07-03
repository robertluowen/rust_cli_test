#!/bin/bash
version="1.1"

# 更新docker的版本号
awk -v version=$version '{gsub(/{VERSION}/, version); print}' .github/workflows/docker.yml.bak > temp && mv temp .github/workflows/docker.yml



