# AsciiFire

[![Github][github-badge]][github-url]
[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/badge/crates.io-v0.1.0-blue
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[github-badge]: https://img.shields.io/badge/github-aaronknudtson/asciifire-brightgreen
[github-url]: https://github.com/aaronknudtson/asciifire
[crates-url]: https://crates.io/crates/asciifire
[mit-url]: https://en.wikipedia.org/wiki/MIT_License

A Library and CLI for converting images into ASCII characters of varying "density".

## Install
Make sure you have cargo installed. [Help can be found here.](https://doc.rust-lang.org/cargo/getting-started/installation.html)

Once cargo is installed, open a terminal and run
```rust
cargo install asciifire
```

This will install a binary and allow you to use asciifire in your preferred terminal.

## Examples

Output to console at appropriate terminal size:
```bash
asciifire Savior.jpeg
```
![Ascii Savior Big](https://github.com/aaronknudtson/asciifire/raw/fe8547d9f4e7d46e6aef40b2b2ea22a92cde40ec/images/SaviorBig.png)

Output result to file:
```bash
asciifire Savior.jpeg -o Savior.txt
```

Resize height and maintain aspect ratio:
```bash
asciifire Savior.jpeg --height 50
```
```shell
I;;IIIIIIIIIIlIlll!!!!!!!!iiii>>>>>i>>>>>>>>iiiiiiiii!!!!llllIIIIIIII;;;;;
II;I;IIIIIIIllll!!!!!!!iiiii>>>>>>>>><<>>>>>>>>>>i>i>i!!!llllIIIIIII;;;;;;
lIIIIIIIlllll!!!iiiii>>>>>>>~~~~~~~~~~~~~~~<<>><<<<>>>>i!!!!llllIlIII;III;
lIIIIllll!!!!!iiiii>><~~~++++++~~~~~~~~~+~~++~~<<~~<<<<>ii!!!!!!llllI;IIII
llllIl!!!iiiiiii>>><~~+_-_++++++?[}1)1}?-+++++++~~~~~~~<>>>iii!!!!lllIIIII
!llll!!!iiiiii>>><~++-?--__++?1tjjxcLZJvuf{_+++__+~++~~~<<<>iiii!!!!lIIIII
!l!!!!iiii>>>>><~++_-??-----{fvXCYvuYLYvJJUj?_+___+~++~~~~<>>>iii!!!!lllll
!l!!!!iii>>>><<~~+_-------?|cUZLf?~<<~_{fvUOx-+++__++++~~~~<<>>ii!!!!ll!!!
!!!!!!iii>>>><<~+__-?-__--(ULmmt~!!!!!i~]|Xq01+++___~_++~~~<<<>iii!!!l!!l!
!!!!!iii>>><<<~++__--___-[vLmOu//fx\-<<-{\uwac-+++++++++~~~<<>>>iiii!!!!!l
!!!!!iii>>><<~~++_---___?xCmqx|rXCQv[+fUCYJdac_+++++~+++~~~~~<>>>>ii!!!!!!
!!!!!!ii>><<<~~+__---___{XOd0(~<+[}_>-UXJCLhkc_++++~~++~~~~~~<<>>iiii!!!!!
!!!!!!ii>>><<~~+++_--__?/vQ0zf[+<><~>{x}[\Coqz]++~~<+++++~~~~<<<>>iii!!!!!
!!!!!iii>>><<<<~~+__-_-\vJbdOn|}?--|jXv)tYd*dU[~~~~~++++~~~~<<<<>>>ii!!!!!
!!!!!!iii>>>><<~~+__---tX0hohCufrxnuzLLuJdoaLc|+~~+++++++~~~~<<<>>>>i!!!!!
!!!!!!i!ii>>>><~~+___-]jJwphkUJYXf/xzUQmpoobZzc-+~++++++~~~~~~<<<>>ii!!!!!
!!!!!!iii>>>><<~++___?)uJLQahvtzXvvccLwdo#abbmX[_+++++++~~~~~~<<<>>>i!!!!!
!l!!!!iii>>><~~~~++++{jCZZbabv{)nYLQCqwwa#oqZdmX(-_+___+++~~~~<<<<>>ii!!!!
l!!!!ii>>><<~~~~++++{u0dmZkdJr1?{\\rzJJQwphpwwOqmn}-____+++~~~~~<<>>ii!!!!
!!!iii>>><<<~~~~~++)YLpmLpwn({}]])|/fnzUUYU0wmqbO0Jf{]-__++~~~~~<<>>ii!i!!
!!iiii>><<<<~~~+-)rczYLwOCu\[~~++{{?}\/tjruXLZpOcUZOJcj)-++++~~<<<>>ii!!!i
iiiiii>>><<~~+]\xzzXXULxunnj/){[][}{)(|fxvzYXJCXcYCZQCYz)_+~~~~<<>>>iiiii!
iiiii>>><<~~_(uvucXcJLY\jtjxxxxrjfjjfjrxuuuczYUnvXY00Yzcz)_++~~<<<<>>iiiii
iiiii>>><~+-\zcvvccYCu|frjftffftjffftttjjtrucYCxcXXLQYcz0C|_++~~~<<>>iiiii
iiiii><<~+-jUXczzczYj({tuu(t\|//|/fx/|\()\tnXYJuzccJLYzXmmJ/-+~~~<<<>iiiii
iii>i><<~-jCJYcXcvYYxf)|rt(r/(1(((/n\({[{(ruuuznvccYJXXYZ00Lr]+~~<<<>iiiii
iii>>><~_/CQLcXUnzQYf/{}{-tjt\)}[))\}({?}((|/jvxnuczUYXC0UCO0c}+~~<<>iiiii
>>>>><~+)U00YJQXnXLv-]{+<+xr/(1}{{\f/\1]??[1||/rxxvzYXYQmLQ0ZOz{+~<<>ii>ii
>>>>><+}XLCCQYvxjuJu?>_~!?xrt|)\rxxxxucx|\/ffnnxxxnXCJJ0ZOCCLLLc1-~~<>>>>i
>>><<~?xzXO0crnvczUYf-+[(rxt|){})jrxnnt[{ttrjvcczunzXzzcnYuxjxvYcr)+<>>>>>
<<<<<_\ucJCYurXCmLUUUf)xXXcnf|}?-?1/f{-_?1tjjxucYzvzcvunrjuUzvczLLX/->>>>>
>><~_\XvCmwQ0Q0QUzXYn|vLCUcr/(1]?[?[?[}{[1\ruuYYUYXUYXcuxfjJ0CXzYCUcj->>>>
>><+)zJvmZ0LqmLLUccuncuUXzurj/}{)[}1?|\xj/rnczYXUXzULLCYzccOQOXzzJYzc)<>>>
>>~{nJUzwLCUOZ0XXccXLYxYYYYcf\/x()/\{/jxXcucXYcvUvtXZqwQmwZZLQUvcCOJX\<>>>
><_fcJXXmCCJOOOXxUZwZJnCOmUnxuzrffxjtjxnXUUYUUzXzfnCmkqOpkbZCLLcXXOOX\<>>>
<<_jzCcvmQCCZOQUxYbdOCYmmLUXJCcunuccuuuuzCO0JJcvucJQqhwc0bw0LCLcUXcQz1<>>i
<<~}cQYuZqQLZZCUccmqQJX0OZZZ0YXcczCJXUzXXCZZLurvYCL0dhZuLd0Q0UYvJYuXu?<>>>
<~~_fLYuQbZL0mCUXXYZCYvOddbdLLJYJCwZUQCJJLmmvrcYCLQZpbZuLwU0ZXccUXnnf~<>>i
<<~+1YznJdZCLOCYYUzJUzcwddd0L0JLQ0qZJ0QCCLUvuvYCCCmddpZYU0XZZXYYYcxj/~<>>i
<<~+[zXvYpOCCQLYYJUcUzzwpZ0LO0L0QOm0J00CJcjxvYYXUZbkp0Z0JLYmOYYXznrf/]<<>>
<~~+-nUYYqZCLLQQJLOUYXCpwQCLOQQO0O0LCQQUvjxvzcvz0dkdLQmwQQQO0YYzvxjf/[~<>>
<~~+_/YCUmdOQ0OOmOOOQQZqOLCL0LQO0OQCJJvrjnczcvcQdbdQC0mdbddpqYcvnrjt{+~<>>
<<~~+}vJUQhkpwqdbhbddqwm0LCLQLQ0QOLJvfjxuccvvzLdkd0JQmbhhaakbCunrxc(+~<<>>
~~~++-/UJUpkhkpbbddpqZO0QCCLLCL0Q0CujxzYYXnnzLphdCzJZbbbhhhddZuruLC)+~<<>>
~~~++_[nUzObbbpO0OQYvuuvczXUJJCLCUXcXULCYzxcOpbpCcUZbkqbhhhqqqzzL0z)+~<<>>
~~~++__}vcULCCQQCJUYzuxjffjxnnunuucvcvczzUQwpq0XcCmwmZwkhdhdZZUQCUn1+~<<>>
~~~++++_(nvnnnucXUUzcvunrjjjrxnnuunncUL0Zwqm0UcYL0OLCmkbkqkdZOCUUzx)~<>>ii
~~~+++_-?1txunrrxuzzzzcunrjjxxxxnucXUC0O00QUzzUJYYYJZbbwZwkqZZQYrrr1~<>>ii
~~~~~+__-??[)xcccvvvvcvunxf/fxnuuccvucYCCUccYUXzzXXQbbmCLwbZZQUx?-?-<<>>ii
<<~~~++_-???]jcJCQOQLQQLCCJYzvuvccvuvzXzvvzXXzczXXLqqOCYCqp0QCct-++~<>>>ii
```


![Ascii Savior 50](https://raw.githubusercontent.com/aaronknudtson/asciifire/images/Savior50.png)
