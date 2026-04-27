dice

A personal command line dice roller written in Rust, designed for TTRPG use.

Usage:
dice <no> <sides> [modifier]
dice <no> <sides> --test
	<no> -- number of dice to roll
	<sides> -- number of sides on each die
	[modifier] -- optional, if present dice will be totaled together with modifier

Use dice -h to print help. 

Examples:
	dice 1 20
	dice 5 6 +4
	dice 1000 8 --test

Test Mode: 
Test mode rolls the specified dice and calculates a chi squared statistic, useful 
for verifying the fairness of the RNG. Designed to support very large roll counts.

More features in development.


