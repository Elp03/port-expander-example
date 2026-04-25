Test for checking the PCAL9714

The tests are done with using a nrf9151 and the PCAL9714. 
Testing the signals a logic analyser was used. 
The PCAL9714 was soldered to a breakout board which slotted into a breadboard for easy accesing. (And also because there was something wrong with my custom PCB :02cry:)

Implemented | Hardware tested | Branch | Note 
--- |--- |--- |--- |
configure input/output | [x] | NA | |
set high/low | [x] | main | |
is high/low?  | [x] | pcal-input-test | | 
Sett pullup    | [ ] | pcal-pullup | Internal pullup 100k, one extern 100k to earth, measure then 3v3/2 = 1.65V by voltage dividing |
Sett pulldown  | [ ] | | |
Sett polarity inversion  | [ ] | | |
