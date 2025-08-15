#!/bin/sh

clear
rm -fr logs/*.log
rm -fr temp/*
cargo b --release;
./target/release/comparador -d datasets/open-images-dataset-v7/test_challenge_2018
    # 2> logs.log
