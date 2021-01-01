#!/bin/bash

if [ ! -d "./dist" ]
then
  mkdir dist
fi

cp ./index.html ./dist/index.html
cp ./tailwind.css ./dist/tailwind.css
cp -r ./assets ./dist/assets
