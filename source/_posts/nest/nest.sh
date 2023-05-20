#! /bin/bash

read -p "enter file name:" fileName


touch $fileName\.md

echo "---
title: ${fileName}
categories: Nest
---" >> $fileName\.md

