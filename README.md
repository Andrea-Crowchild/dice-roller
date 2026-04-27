This is just a simple dice roller I'm writing for myself. 
Syntax is "dice <no> <sides> <modifier>" where <no> is the number of dice and
<sides> is the number of sides, the optional <modifier> will add to the rolled dice and
present a total. Use "dice -h" to print the help dialogue.
Use "dice <no> <sides> --test" to engage test mode. Test mode will calculate chi-squared
for the rolled dice. 

 
Example:
dice 1 20
dice 5 6 +3
dice 1000 6 --test
Am going to be adding features like modifiers later. 

