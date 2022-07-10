#!/bin/sh
source $PWD/.env && env | grep RIOT_ID
source $PWD/.env && cargo test
