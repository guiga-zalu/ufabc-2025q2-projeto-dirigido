#!/bin/sh

aws s3 --no-sign-request sync \
    s3://open-images-dataset/challenge2018 src/test_challenge_2018
