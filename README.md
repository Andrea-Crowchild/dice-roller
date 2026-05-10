dice

A personal command line dice roller written in Rust, designed for TTRPG use.

Example syntax: 
dice 1 8
dice 1 20 +4
dice 1000 6 --test
dice --help

Test Mode: 
Test mode rolls the specified dice and calculates a chi squared statistic, useful 
for verifying the fairness of the RNG. Designed to support very large roll counts.
