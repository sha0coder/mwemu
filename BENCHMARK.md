
# BENCHMARK

Let's emulate 10 million of instructions and measure it 5 times.

The time include PE loading + emulation + memory dump

libmwemu version: 0.21.0

```
processor	: 15
vendor_id	: AuthenticAMD
cpu family	: 25
model		: 117
model name	: AMD Ryzen 9 8945HS w/ Radeon 780M Graphics
stepping	: 2
cpu MHz		: 400.000
cache size	: 1024 KB

~/s/mwemu ❯❯❯ free                  
               total       usado       libre  compartido   búf/caché  disponible
Mem:        63555984    22467896     8691988      497368    32396100    35248148
Inter:      16777212       39424    16737788
```


## 32bits with verbose (-vv)

```bash
time cargo run --release -- -f  test/sc32win_donut.bin -vv -e 10000001
________________________________________________________
Executed in   24,34 secs    fish           external
   usr time    4,60 secs  515,00 micros    4,60 secs
   sys time   19,63 secs  768,00 micros   19,63 secs
________________________________________________________
Executed in   24,70 secs    fish           external
   usr time    4,74 secs  485,00 micros    4,74 secs
   sys time   19,84 secs  229,00 micros   19,84 secs
________________________________________________________
Executed in   24,61 secs    fish           external
   usr time    4,64 secs    0,18 millis    4,64 secs
   sys time   19,86 secs    1,08 millis   19,86 secs
________________________________________________________
Executed in   24,49 secs    fish           external
   usr time    4,54 secs    0,42 millis    4,54 secs
   sys time   19,84 secs    1,19 millis   19,84 secs
________________________________________________________
Executed in   24,53 secs    fish           external
   usr time    4,58 secs    0,00 micros    4,58 secs
   sys time   19,84 secs  828,00 micros   19,84 secs
```

## 32bits with no verbose

```bash
time cargo run --release -- -f  test/sc32win_donut.bin -e 10000001
________________________________________________________
Executed in  703,60 millis    fish           external
   usr time  468,12 millis    0,03 millis  468,10 millis
   sys time  131,87 millis    1,01 millis  130,87 millis
________________________________________________________
Executed in  695,20 millis    fish           external
   usr time  461,86 millis    0,00 micros  461,86 millis
   sys time  129,88 millis  924,00 micros  128,96 millis
________________________________________________________
Executed in  716,83 millis    fish           external
   usr time  465,88 millis  700,00 micros  465,18 millis
   sys time  132,95 millis  334,00 micros  132,62 millis
________________________________________________________
Executed in  686,57 millis    fish           external
   usr time  444,87 millis    0,03 millis  444,85 millis
   sys time  137,49 millis    1,01 millis  136,49 millis
________________________________________________________
Executed in  710,23 millis    fish           external
   usr time  458,69 millis    0,00 micros  458,69 millis
   sys time  141,92 millis  705,00 micros  141,21 millis
```

## 64bits with verbose  (some winapi overhead)

```bash
time cargo run --release -- -f test/sc64win_metasploit.bin -6 -vv -e 10000001 
________________________________________________________
Executed in   27,69 secs    fish           external
   usr time    4,75 secs    0,00 micros    4,75 secs
   sys time   22,78 secs  617,00 micros   22,78 secs
________________________________________________________
Executed in   27,60 secs    fish           external
   usr time    4,86 secs    0,00 millis    4,86 secs
   sys time   22,58 secs    1,50 millis   22,58 secs
________________________________________________________
Executed in   27,99 secs    fish           external
   usr time    4,85 secs    0,09 millis    4,85 secs
   sys time   22,99 secs    1,05 millis   22,99 secs
________________________________________________________
Executed in   27,65 secs    fish           external
   usr time    4,73 secs    0,00 micros    4,73 secs
   sys time   22,76 secs  634,00 micros   22,76 secs
________________________________________________________
Executed in   27,82 secs    fish           external
   usr time    4,82 secs    0,00 micros    4,82 secs
   sys time   22,84 secs  780,00 micros   22,84 secs
```

## 64bits with no verbose  (some winapi overhead)

```bash
time cargo run --release -- -f test/sc64win_metasploit.bin -6  -e 10000001
________________________________________________________
Executed in    2,24 secs    fish           external
   usr time    0,56 secs    0,02 millis    0,56 secs
   sys time    1,53 secs    1,00 millis    1,53 secs
________________________________________________________
Executed in    2,22 secs    fish           external
   usr time    0,55 secs    0,00 millis    0,55 secs
   sys time    1,51 secs    1,07 millis    1,51 secs
________________________________________________________
Executed in    2,27 secs    fish           external
   usr time    0,54 secs  339,00 micros    0,54 secs
   sys time    1,54 secs  260,00 micros    1,54 secs
________________________________________________________
Executed in    2,24 secs    fish           external
   usr time    0,55 secs  356,00 micros    0,55 secs
   sys time    1,52 secs  270,00 micros    1,52 secs
________________________________________________________
Executed in    2,22 secs    fish           external
   usr time    0,53 secs    0,00 micros    0,53 secs
   sys time    1,53 secs  703,00 micros    1,53 secs
```



## max-speed, 32bits with no API calls

18M i 1second

```bash
time cargo run --release -- -f  test/sc32win_donut.bin -e 18000001
________________________________________________________
Executed in    1,04 secs      fish           external
   usr time  762,69 millis    0,00 micros  762,69 millis
   sys time  144,85 millis  473,00 micros  144,37 millis
________________________________________________________
Executed in  983,67 millis    fish           external
   usr time  747,82 millis    0,00 micros  747,82 millis
   sys time  131,10 millis  779,00 micros  130,32 millis
________________________________________________________
Executed in    1,01 secs      fish           external
   usr time  778,74 millis    0,00 micros  778,74 millis
   sys time  130,27 millis  812,00 micros  129,46 millis
________________________________________________________
Executed in    1,01 secs      fish           external
   usr time  766,83 millis    0,00 micros  766,83 millis
   sys time  137,09 millis  657,00 micros  136,44 millis
________________________________________________________
Executed in  985,74 millis    fish           external
   usr time  758,16 millis  498,00 micros  757,66 millis
   sys time  135,46 millis  159,00 micros  135,30 millis
```

