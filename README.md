Test for checking the PCAL9471



Implemented | Hardware tested | Branch | Note 
--- |--- |--- |--- |
configure input/output | [x] | NA | |
set high/low | [x] | main | |
is high/low?  | [x] | pcal-input-test | | 
Sett pullup    | [ ] | pcal-pullup | Internal pullup 100k, one extern 100k to earth, measure then 3v3/2 = 1.65V by voltage dividing |
Sett pulldown  | [ ] | pcal-pulldown | I think one of the pins are overwritten when running. Only the last pin works. So need to be fixed in the io expander driver |
Sett polarity inversion  | [x] | pcal-polarity-inversion | the other pins need to be not floating. Else it will not work propperly |
