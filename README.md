Test for checking the PCAL9471



Implemented | Hardware tested | Branch | Note 
--- |--- |--- |--- |
configure input/output | [x] | NA | is tested in all the other tests |
set high/low | [x] | main | |
is high/low?  | [x] | pcal-input-test | | 
Sett pullup    | [x] | pcal-pullup | Internal pullup 100k, one extern 100k to earth, measure then 3v3/2 = 1.65V by voltage dividing |
Sett pulldown  | [x] | pcal-pulldown | voltage divider, with 100k to match the internal pulldown. The resistor to vdd, and measure between. same as pullup, just to vdd instead of vss |
Sett polarity inversion  | [x] | pcal-polarity-inversion | the other pins need to be not floating. Else it will not work propperly |
