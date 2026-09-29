# K6b tables (from the records; observation only)

b3: slot K6B-S4, source 4eeb206c0, 132 processes. b: pre-KF1, source f4d40dd17, 108 processes.
W1-T4 (10,000 members) is pre-KF3.

## T1. W1 per model and size (b3, pass 1)

| model | members | outcome | attempts | charged LME | stop rule LME (% of call) | shift fact. | profile | pattern | rows | heap MiB | footprint MiB | RSS MiB | call s (median) | ns/LME | load | E_adm MiB | heap/E | rho_fp |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| CHAIN-n00010-AX | 10 | Selected 128 | 128 Accepted, 256 Verified | 9129471 | 1126845 (12.3%) | 1 | 534 | 1116 | 263 | 1.0 | 3.6 | 5.5 | 0.007 | 0.806 | 4.01 | 21.7 | 0.0515 | 0.1147 |
| CHAIN-n00010-ROT | 10 | Selected 128 | 128 Accepted, 256 Verified | 11244502 | 1109238 (9.9%) | 1 | 534 | 1116 | 263 | 1.0 | 3.5 | 5.4 | 0.015 | 1.291 | 4.01 | 21.7 | 0.0513 | 0.1118 |
| TREE-n00010-AX | 10 | Selected 128 | 128 Accepted, 256 Verified | 9088175 | 1298010 (14.3%) | 1 | 534 | 1116 | 263 | 1.0 | 3.4 | 5.3 | 0.008 | 0.892 | 4.01 | 21.7 | 0.0454 | 0.1075 |
| TREE-n00010-ROT | 10 | Selected 128 | 128 Accepted, 256 Verified | 10957856 | 1075268 (9.8%) | 1 | 534 | 1116 | 263 | 0.9 | 3.4 | 5.3 | 0.015 | 1.327 | 4.01 | 21.7 | 0.0425 | 0.1053 |
| CONT-n00010-AX | 10 | Selected 128 | 128 Accepted, 256 Verified | 7103333 | 1473268 (20.7%) | 0 | 297 | 1116 | 278 | 1.2 | 5.4 | 7.3 | 0.007 | 0.932 | 4.08 | 21.6 | 0.0623 | 0.2004 |
| CONT-n00010-ROT | 10 | Selected 128 | 128 Accepted, 256 Verified | 9806965 | 1545960 (15.8%) | 1 | 297 | 1116 | 278 | 1.2 | 4.0 | 5.9 | 0.012 | 1.226 | 4.08 | 21.6 | 0.0622 | 0.1360 |
| DEC053 cantilever-chain-8 | 8 | Selected 128 | 128 Accepted, 256 Verified | 7295927 | 1353844 (18.6%) | 1 | 420 | 900 | 213 | 0.9 | 3.3 | 5.2 | 0.005 | 0.668 | 4.08 | 21.1 | 0.0455 | 0.1043 |
| DEC053 cantilever-chain-24 | 24 | Selected 128 | 128 Accepted, 256 Verified | 21942177 | 3806363 (17.3%) | 1 | 1332 | 2628 | 613 | 2.7 | 9.2 | 11.0 | 0.015 | 0.703 | 4.08 | 25.4 | 0.1171 | 0.3175 |
| DEC053 cantilever-chain-48 | 48 | Selected 128 | 128 Accepted, 256 Verified | 43931984 | 7542846 (17.2%) | 1 | 2700 | 5220 | 1213 | 5.4 | 8.7 | 10.6 | 0.030 | 0.672 | 4.08 | 31.9 | 0.1845 | 0.2373 |
| DEC053 grid-frame-4x3 | 17 | Selected 128 | 128 Accepted, 256 Verified | 11544479 | 1586088 (13.7%) | 1 | 636 | 1656 | 414 | 1.4 | 4.1 | 6.0 | 0.010 | 0.863 | 4.08 | 23.1 | 0.0604 | 0.1276 |
| DEC053 grid-frame-6x8 | 82 | Selected 128 | 128 Accepted, 256 Verified | 85805999 | 6397333 (7.5%) | 1 | 7758 | 7632 | 1848 | 6.9 | 12.6 | 14.5 | 0.078 | 0.904 | 4.08 | 40.2 | 0.1703 | 0.2860 |
| DEC053 grid-frame-7x8 | 97 | Selected 128 | 128 Accepted, 256 Verified | 106575518 | 7441411 (7.0%) | 1 | 9849 | 9000 | 2180 | 8.2 | 14.3 | 16.2 | 0.097 | 0.906 | 4.24 | 44.3 | 0.1860 | 0.2989 |
| DEC053 grid-frame-5x5 | 40 | Selected 128 | 128 Accepted, 256 Verified | 34879241 | 3317605 (9.5%) | 1 | 2760 | 3780 | 925 | 3.2 | 6.4 | 8.3 | 0.031 | 0.884 | 4.24 | 29.1 | 0.1096 | 0.1816 |
| DEC053 cantilever-chain-32 | 32 | Selected 128 | 128 Accepted, 256 Verified | 29473702 | 5245362 (17.8%) | 1 | 1788 | 3492 | 813 | 3.4 | 6.6 | 8.4 | 0.020 | 0.672 | 4.24 | 27.6 | 0.1322 | 0.1975 |
| DEC053 grid-frame-5x6 | 49 | Selected 128 | 128 Accepted, 256 Verified | 45338910 | 4048499 (8.9%) | 1 | 3765 | 4608 | 1122 | 4.0 | 7.6 | 9.5 | 0.041 | 0.897 | 4.24 | 31.4 | 0.1278 | 0.2057 |
| CHAIN-n00100-AX | 100 | Selected 128 | 128 Accepted, 256 Verified | 89784810 | 8505628 (9.5%) | 1 | 5664 | 10836 | 2513 | 10.9 | 16.5 | 18.4 | 0.072 | 0.798 | 4.38 | 46.0 | 0.2605 | 0.3353 |
| CHAIN-n00100-ROT | 100 | Selected 128 | 128 Accepted, 256 Verified | 111923940 | 8486914 (7.6%) | 1 | 5664 | 10836 | 2513 | 10.9 | 16.5 | 18.3 | 0.145 | 1.297 | 4.38 | 46.0 | 0.2604 | 0.3343 |
| TREE-n00100-AX | 100 | Selected 128 | 128 Accepted, 256 Verified | 90125790 | 10647843 (11.8%) | 1 | 5664 | 10836 | 2513 | 9.4 | 13.5 | 15.3 | 0.077 | 0.857 | 4.19 | 46.0 | 0.2093 | 0.2687 |
| TREE-n00100-ROT | 100 | Selected 128 | 128 Accepted, 256 Verified | 108591655 | 8460174 (7.8%) | 1 | 5664 | 10836 | 2513 | 8.8 | 13.3 | 15.1 | 0.138 | 1.269 | 4.19 | 46.0 | 0.1976 | 0.2638 |
| CONT-n00100-AX | 100 | Selected 128 | 128 Accepted, 256 Verified | 70051193 | 13849213 (19.8%) | 0 | 3132 | 10836 | 2663 | 10.7 | 18.9 | 20.7 | 0.059 | 0.847 | 4.17 | 45.3 | 0.2587 | 0.3932 |
| CONT-n00100-ROT | 100 | Selected 128 | 128 Accepted, 256 Verified | 93367469 | 14709661 (15.8%) | 1 | 3132 | 10836 | 2663 | 10.7 | 15.9 | 17.7 | 0.114 | 1.224 | 4.17 | 45.3 | 0.2587 | 0.3261 |
| CHAIN-n01000-AX | 1000 | Selected 128 | 128 Accepted, 256 Verified | 1013371056 | 198796321 (19.6%) | 1 | 56964 | 108036 | 25013 | 85.8 | 112.5 | 121.2 | 0.785 | 0.775 | 4.17 | 289.3 | 0.2967 | 0.3850 |
| CHAIN-n01000-ROT | 1000 | Selected 128 | 128 Accepted, 256 Verified | 1236879507 | 198702925 (16.1%) | 1 | 56964 | 108036 | 25013 | 85.8 | 112.5 | 128.2 | 1.523 | 1.232 | 4.34 | 289.3 | 0.2967 | 0.3850 |
| TREE-n01000-AX | 1000 | Selected 128 | 128 Accepted, 256 Verified | 922653051 | 121940308 (13.2%) | 1 | 56964 | 108036 | 25013 | 85.9 | 124.9 | 126.7 | 0.800 | 0.867 | 4.59 | 289.4 | 0.2968 | 0.4279 |
| TREE-n01000-ROT | 1000 | Selected 128 | 128 Accepted, 256 Verified | 1114735751 | 109153161 (9.8%) | 1 | 56964 | 108036 | 25013 | 86.0 | 126.0 | 127.8 | 1.393 | 1.250 | 5.13 | 289.5 | 0.2971 | 0.4315 |
| CONT-n01000-AX | 1000 | Selected 128 | 128 Accepted, 256 Verified | 806819446 | 245487770 (30.4%) | 0 | 31482 | 108036 | 26513 | 81.2 | 114.0 | 115.8 | 0.608 | 0.753 | 6.01 | 282.0 | 0.2879 | 0.4003 |
| CONT-n01000-ROT | 1000 | Selected 128 | 128 Accepted, 256 Verified | 1042064338 | 254278921 (24.4%) | 1 | 31482 | 108036 | 26513 | 81.3 | 117.5 | 119.3 | 1.198 | 1.150 | 5.41 | 282.1 | 0.2881 | 0.4127 |
| CHAIN-n10000-AX | 10000 | Unresolved(ExactSumSpan) | 128 Rejected, 256 Failed | 6835124231 | 0 (0.0%) | - | 569964 | 1080036 | - | 831.7 | 889.1 | 986.1 | 4.968 | 0.727 | 5.46 | 2724.2 | 0.3053 | 0.3260 |
| CHAIN-n10000-ROT | 10000 | Unresolved(ExactSumSpan) | 128 Rejected, 256 Failed | 8386149922 | 0 (0.0%) | - | 569964 | 1080036 | - | 831.7 | 886.4 | 983.5 | 10.092 | 1.203 | 4.79 | 2724.2 | 0.3053 | 0.3250 |
| TREE-n10000-AX | 10000 | Unresolved(ExactSumSpan) | 128 Rejected, 256 Failed | 6674734550 | 0 (0.0%) | - | 569964 | 1080036 | - | 832.5 | 874.6 | 978.2 | 5.177 | 0.776 | 3.68 | 2725.1 | 0.3055 | 0.3205 |
| TREE-n10000-ROT | 10000 | Unresolved(ExactSumSpan) | 128 Rejected, 256 Failed | 8194818095 | 0 (0.0%) | - | 569964 | 1080036 | - | 833.9 | 863.7 | 963.9 | 9.555 | 1.166 | 3.66 | 2726.8 | 0.3058 | 0.3163 |
| CONT-n10000-AX | 10000 | Selected 128 | 128 Accepted, 256 Verified | 8234772320 | 2596783796 (31.5%) | 0 | 314982 | 1080036 | 265013 | 814.0 | 995.4 | 1061.4 | 6.090 | 0.740 | 4.54 | 2649.1 | 0.3073 | 0.3753 |
| CONT-n10000-ROT | 10000 | Unresolved(ExactSumSpan) | 128 Rejected, 256 Failed | 6884609317 | 0 (0.0%) | - | 314982 | 1080036 | - | 814.9 | 903.8 | 965.9 | 8.134 | 1.181 | 4.94 | 2650.3 | 0.3075 | 0.3406 |

## T2. Work by precision and part (b3, pass 1, repeat 0; LME)

Parts: solve = rhs + solve + refinement + recovery; build = formation + assembly + residual formation + factor + condition; pass = the verification pass (scale, estimate, charge, bound, shift); v-build = the verification's shared data (bounded formation, wide formation, uc); unstaged = charged work no stage records (a stopped build).

| model | p | role | outcome | solve | stop rule | pass | build | v-build (bounded / wide / uc) | unstaged | charged by |
|---|---|---|---|---|---|---|---|---|---|---|
| CHAIN-n00010-AX | 128 | Candidate | Accepted | 293219 | 1126845 | 0 | 2074272 | 0 / 0 / 0 | 0 | 3494336 |
| CHAIN-n00010-AX | 256 | Verification | Verified | 330698 | 0 | 1098562 | 2749799 | 360888 / 939383 / 155805 | 0 | 5635135 |
| CHAIN-n00010-ROT | 128 | Candidate | Accepted | 388076 | 1109238 | 0 | 2378613 | 0 / 0 / 0 | 0 | 3875927 |
| CHAIN-n00010-ROT | 256 | Verification | Verified | 504669 | 0 | 1479840 | 3289477 | 360888 / 1319209 / 414492 | 0 | 7368575 |
| TREE-n00010-AX | 128 | Candidate | Accepted | 314274 | 1298010 | 0 | 1935164 | 0 / 0 / 0 | 0 | 3547448 |
| TREE-n00010-AX | 256 | Verification | Verified | 343918 | 0 | 1127125 | 2610953 | 360888 / 939396 / 158447 | 0 | 5540727 |
| TREE-n00010-ROT | 128 | Candidate | Accepted | 396458 | 1075268 | 0 | 2239962 | 0 / 0 / 0 | 0 | 3711688 |
| TREE-n00010-ROT | 256 | Verification | Verified | 529442 | 0 | 1479613 | 3152173 | 360888 / 1318748 / 405304 | 0 | 7246168 |
| CONT-n00010-AX | 128 | Candidate | Accepted | 194739 | 1473268 | 0 | 1422009 | 0 / 0 / 0 | 0 | 3090016 |
| CONT-n00010-AX | 256 | Verification | Verified | 222041 | 0 | 290034 | 2096270 | 360888 / 939383 / 104701 | 0 | 4013317 |
| CONT-n00010-ROT | 128 | Candidate | Accepted | 293541 | 1545960 | 0 | 2004507 | 0 / 0 / 0 | 0 | 3844008 |
| CONT-n00010-ROT | 256 | Verification | Verified | 362004 | 0 | 906963 | 2776538 | 360888 / 1319209 / 237355 | 0 | 5962957 |
| DEC053 cantilever-chain-8 | 128 | Candidate | Accepted | 160919 | 1353844 | 0 | 1631329 | 0 / 0 / 0 | 0 | 3146092 |
| DEC053 cantilever-chain-8 | 256 | Verification | Verified | 174581 | 0 | 770387 | 2156052 | 217716 / 730578 / 100521 | 0 | 4149835 |
| DEC053 cantilever-chain-24 | 128 | Candidate | Accepted | 446080 | 3806363 | 0 | 5041157 | 0 / 0 / 0 | 0 | 9293600 |
| DEC053 cantilever-chain-24 | 256 | Verification | Verified | 480690 | 0 | 2421120 | 6601456 | 651924 / 2191738 / 301649 | 0 | 12648577 |
| DEC053 cantilever-chain-48 | 128 | Candidate | Accepted | 851403 | 7542846 | 0 | 10155925 | 0 / 0 / 0 | 0 | 18550174 |
| DEC053 cantilever-chain-48 | 256 | Verification | Verified | 942523 | 0 | 4876109 | 13269717 | 1303236 / 4383478 / 606747 | 0 | 25381810 |
| DEC053 grid-frame-4x3 | 128 | Candidate | Accepted | 219241 | 1586088 | 0 | 2424903 | 0 / 0 / 0 | 0 | 4230232 |
| DEC053 grid-frame-4x3 | 256 | Verification | Verified | 240599 | 0 | 1282205 | 3529180 | 458286 / 1552575 / 251402 | 0 | 7314247 |
| DEC053 grid-frame-6x8 | 128 | Candidate | Accepted | 1272911 | 6397333 | 0 | 21520399 | 0 / 0 / 0 | 0 | 29190643 |
| DEC053 grid-frame-6x8 | 256 | Verification | Verified | 1403047 | 0 | 15810444 | 26820320 | 2204508 / 7489054 / 2887983 | 0 | 56615356 |
| DEC053 grid-frame-7x8 | 128 | Candidate | Accepted | 1522039 | 7441411 | 0 | 27100734 | 0 / 0 / 0 | 0 | 36064184 |
| DEC053 grid-frame-7x8 | 256 | Verification | Verified | 1675614 | 0 | 20336675 | 33368613 | 2607294 / 8859061 / 3664077 | 0 | 70511334 |
| DEC053 grid-frame-5x5 | 128 | Candidate | Accepted | 572452 | 3317605 | 0 | 8245372 | 0 / 0 / 0 | 0 | 12135429 |
| DEC053 grid-frame-5x5 | 256 | Verification | Verified | 632158 | 0 | 5506472 | 10834375 | 1076340 / 3653200 / 1041267 | 0 | 22743812 |
| DEC053 cantilever-chain-32 | 128 | Candidate | Accepted | 571654 | 5245362 | 0 | 6746089 | 0 / 0 / 0 | 0 | 12563105 |
| DEC053 cantilever-chain-32 | 256 | Verification | Verified | 626328 | 0 | 3264544 | 8824206 | 869028 / 2922318 / 404173 | 0 | 16910597 |
| DEC053 grid-frame-5x6 | 128 | Candidate | Accepted | 724698 | 4048499 | 0 | 10917530 | 0 / 0 / 0 | 0 | 15690727 |
| DEC053 grid-frame-5x6 | 256 | Verification | Verified | 799742 | 0 | 7551778 | 14087515 | 1318134 / 4475158 / 1415856 | 0 | 29648183 |
| CHAIN-n00100-AX | 128 | Candidate | Accepted | 2642875 | 8505628 | 0 | 21404659 | 0 / 0 / 0 | 0 | 32553162 |
| CHAIN-n00100-AX | 256 | Verification | Verified | 3042010 | 0 | 11483480 | 28096093 | 3601428 / 9393848 / 1614789 | 0 | 57231648 |
| CHAIN-n00100-ROT | 128 | Candidate | Accepted | 3743255 | 8486914 | 0 | 24447603 | 0 / 0 / 0 | 0 | 36677772 |
| CHAIN-n00100-ROT | 256 | Verification | Verified | 4972220 | 0 | 15552169 | 33489702 | 3601428 / 13192144 / 4438505 | 0 | 75246168 |
| TREE-n00100-AX | 128 | Candidate | Accepted | 2977910 | 10647843 | 0 | 20100797 | 0 / 0 / 0 | 0 | 33726550 |
| TREE-n00100-AX | 256 | Verification | Verified | 3244788 | 0 | 11829575 | 26795246 | 3601428 / 9394023 / 1534180 | 0 | 56399240 |
| TREE-n00100-ROT | 128 | Candidate | Accepted | 3839849 | 8460174 | 0 | 22747412 | 0 / 0 / 0 | 0 | 35047435 |
| TREE-n00100-ROT | 256 | Verification | Verified | 5242579 | 0 | 15487312 | 31802211 | 3601428 / 13187750 / 4222940 | 0 | 73544220 |
| CONT-n00100-AX | 128 | Candidate | Accepted | 1885492 | 13849213 | 0 | 14287448 | 0 / 0 / 0 | 0 | 30022153 |
| CONT-n00100-AX | 256 | Verification | Verified | 2098782 | 0 | 2891267 | 20966226 | 3601428 / 9393848 / 1077489 | 0 | 40029040 |
| CONT-n00100-ROT | 128 | Candidate | Accepted | 2782905 | 14709661 | 0 | 17302751 | 0 / 0 / 0 | 0 | 34795317 |
| CONT-n00100-ROT | 256 | Verification | Verified | 3504166 | 0 | 9408507 | 26329115 | 3601428 / 13192144 / 2536792 | 0 | 58572152 |
| CHAIN-n01000-AX | 128 | Candidate | Accepted | 26108889 | 198796321 | 0 | 214708433 | 0 / 0 / 0 | 0 | 439613643 |
| CHAIN-n01000-AX | 256 | Verification | Verified | 29805515 | 0 | 115522606 | 281559094 | 36006828 / 93938498 / 16924872 | 0 | 573757413 |
| CHAIN-n01000-ROT | 128 | Candidate | Accepted | 37327954 | 198702925 | 0 | 245137500 | 0 / 0 / 0 | 0 | 481168379 |
| CHAIN-n01000-ROT | 256 | Verification | Verified | 49688186 | 0 | 156136850 | 335491939 | 36006828 / 131921494 / 46465831 | 0 | 755711128 |
| TREE-n01000-AX | 128 | Candidate | Accepted | 29755752 | 121940308 | 0 | 201668945 | 0 / 0 / 0 | 0 | 353365005 |
| TREE-n01000-AX | 256 | Verification | Verified | 34514082 | 0 | 120255600 | 268539435 | 36006828 / 93940248 / 16031853 | 0 | 569288046 |
| TREE-n01000-ROT | 128 | Candidate | Accepted | 38303000 | 109153161 | 0 | 227821836 | 0 / 0 / 0 | 0 | 375277997 |
| TREE-n01000-ROT | 256 | Verification | Verified | 52395910 | 0 | 156156558 | 318302103 | 36006828 / 131877608 / 44718747 | 0 | 739457754 |
| CONT-n01000-AX | 128 | Candidate | Accepted | 18029964 | 245487770 | 0 | 142941410 | 0 / 0 / 0 | 0 | 406459144 |
| CONT-n01000-AX | 256 | Verification | Verified | 20759423 | 0 | 28837176 | 209663750 | 36006828 / 93938498 / 11154627 | 0 | 400360302 |
| CONT-n01000-ROT | 128 | Candidate | Accepted | 27678112 | 254278921 | 0 | 173095299 | 0 / 0 / 0 | 0 | 455052332 |
| CONT-n01000-ROT | 256 | Verification | Verified | 34941394 | 0 | 94782413 | 263295499 | 36006828 / 131921494 / 26064378 | 0 | 587012006 |
| CHAIN-n10000-AX | 128 | Candidate | Rejected(VerificationFailed) | 259517786 | 0 | 0 | 2147746443 | 0 / 0 / 0 | 0 | 2407264229 |
| CHAIN-n10000-AX | 256 | Verification | Failed(Stop(Span)) | 297507618 | 0 | 0 | 2816190317 | 360060828 / 939384998 / 0 | 14716241 | 4427860002 |
| CHAIN-n10000-ROT | 128 | Candidate | Rejected(VerificationFailed) | 373220067 | 0 | 0 | 2452036614 | 0 / 0 / 0 | 0 | 2825256681 |
| CHAIN-n10000-ROT | 256 | Verification | Failed(Stop(Span)) | 496996537 | 0 | 0 | 3355514267 | 360060828 / 1319214994 / 0 | 29106615 | 5560893241 |
| TREE-n10000-AX | 128 | Candidate | Rejected(VerificationFailed) | 296112910 | 0 | 0 | 2017353381 | 0 / 0 / 0 | 0 | 2313466291 |
| TREE-n10000-AX | 256 | Verification | Failed(Stop(Span)) | 343729014 | 0 | 0 | 2686092122 | 360060828 / 939402498 / 0 | 31983797 | 4361268259 |
| TREE-n10000-ROT | 128 | Candidate | Rejected(VerificationFailed) | 382991767 | 0 | 0 | 2314419160 | 0 / 0 / 0 | 0 | 2697410927 |
| TREE-n10000-ROT | 256 | Verification | Failed(Stop(Span)) | 524096964 | 0 | 0 | 3219154324 | 360060828 / 1318776233 / 0 | 75318819 | 5497407168 |
| CONT-n10000-AX | 128 | Candidate | Accepted | 178178964 | 2596783796 | 0 | 1429481990 | 0 / 0 / 0 | 0 | 4204444750 |
| CONT-n10000-AX | 256 | Verification | Verified | 233076239 | 0 | 289109150 | 2096643784 | 360060828 / 939384998 / 112052571 | 0 | 4030327570 |
| CONT-n10000-ROT | 128 | Candidate | Rejected(VerificationFailed) | 276625079 | 0 | 0 | 1811687868 | 0 / 0 / 0 | 0 | 2088312947 |
| CONT-n10000-ROT | 256 | Verification | Failed(Stop(Span)) | 349350963 | 0 | 0 | 2713626046 | 360060828 / 1319214994 / 0 | 54043539 | 4796296370 |

## T3. Memory per precision: the prefixes (b3, pass 1; heap above the call's start, MiB; time s)

| model | solve_128 | solve_256 | verify_256 | call | last segment | stop-rule increment | kept-entry equivalent |
|---|---|---|---|---|---|---|---|
| CHAIN-n01000-AX | 35.7 (+35.7), 0.17 s | 61.8 (+26.1), 0.37 s | 85.3 (+23.5), 0.65 s | 85.3, 0.79 s | 0.0 | 0.0 | 0 |
| CHAIN-n01000-ROT | 35.7 (+35.7), 0.41 s | 61.8 (+26.1), 0.82 s | 85.3 (+23.5), 1.37 s | 85.3, 1.52 s | 0.0 | 0.0 | 0 |
| TREE-n01000-AX | 35.7 (+35.7), 0.20 s | 61.8 (+26.1), 0.38 s | 85.4 (+23.5), 0.68 s | 85.4, 0.80 s | 0.0 | 0.0 | 0 |
| TREE-n01000-ROT | 35.8 (+35.8), 0.35 s | 61.9 (+26.1), 0.74 s | 85.5 (+23.5), 1.26 s | 85.5, 1.39 s | 0.0 | 0.0 | 0 |
| CONT-n01000-AX | 34.5 (+34.5), 0.11 s | 60.6 (+26.1), 0.24 s | 80.6 (+20.0), 0.45 s | 80.6, 0.61 s | 0.0 | 0.0 | 0 |
| CONT-n01000-ROT | 34.6 (+34.6), 0.27 s | 60.7 (+26.1), 0.60 s | 80.7 (+20.0), 0.97 s | 80.7, 1.20 s | 0.0 | 0.0 | 0 |
| CHAIN-n10000-AX | 351.6 (+351.6), 1.69 s | 612.6 (+261.0), 3.74 s | - | 826.0, 4.97 s | 213.4 | - | - |
| CHAIN-n10000-ROT | 351.6 (+351.6), 3.82 s | 612.6 (+261.0), 8.06 s | - | 826.0, 10.09 s | 213.4 | - | - |
| TREE-n10000-AX | 352.0 (+352.0), 2.00 s | 613.0 (+261.0), 3.86 s | - | 826.4, 5.18 s | 213.4 | - | - |
| TREE-n10000-ROT | 352.8 (+352.8), 3.49 s | 613.8 (+261.0), 7.51 s | - | 827.2, 9.56 s | 213.4 | - | - |
| CONT-n10000-AX | 346.1 (+346.1), 1.10 s | 607.1 (+261.0), 2.47 s | 807.4 (+200.2), 4.52 s | 807.4, 6.09 s | 0.0 | 0.0 | 0 |
| CONT-n10000-ROT | 346.7 (+346.7), 2.78 s | 607.7 (+261.0), 6.08 s | - | 807.9, 8.13 s | 200.2 | - | - |

## T4. Before and after KF1 at 1,000 members (pass 1): b (pre-KF1) against b3

| model | stop rule LME b | b3 | call heap MiB b | b3 | stop-rule increment MiB b | b3 | kept equivalent b | b3 | call s b | b3 |
|---|---|---|---|---|---|---|---|---|---|---|
| CHAIN-n01000-AX | 82276385 | 198796321 | 102.1 | 85.3 | 16.8 | 0.0 | 4088 | 0 | 0.84 | 0.79 |
| CHAIN-n01000-ROT | 82182989 | 198702925 | 102.1 | 85.3 | 16.7 | 0.0 | 4075 | 0 | 1.58 | 1.52 |
| TREE-n01000-AX | 104014164 | 121940308 | 89.5 | 85.4 | 4.1 | 0.0 | 1005 | 0 | 0.79 | 0.80 |
| TREE-n01000-ROT | 82263945 | 109153161 | 85.5 | 85.5 | 0.0 | 0.0 | 0 | 0 | 1.38 | 1.39 |
| CONT-n01000-AX | 137930906 | 245487770 | 116.6 | 80.6 | 36.0 | 0.0 | 8773 | 0 | 0.56 | 0.61 |
| CONT-n01000-ROT | 146722057 | 254278921 | 116.7 | 80.7 | 36.0 | 0.0 | 8774 | 0 | 1.15 | 1.20 |

## T5. W1/binary64 time multiple per pair (median w1_solve over the sparse median entry_checked, and over the sparse staged total; b3)

| model | pass 1 entry | pass 1 staged | pass 2 entry | pass 2 staged | W1 outcome |
|---|---|---|---|---|---|
| CHAIN-n00010-AX | 8.7 | 6.9 | 8.7 | 6.8 | Selected 128 |
| CHAIN-n00010-ROT | 11.4 | 9.5 | 11.8 | 9.6 | Selected 128 |
| CHAIN-n00100-AX | 9.4 | 7.4 | 9.4 | 7.4 | Selected 128 |
| CHAIN-n00100-ROT | 13.3 | 11.3 | 13.3 | 11.3 | Selected 128 |
| CHAIN-n01000-AX | 10.2 | 8.0 | 10.3 | 8.1 | Selected 128 |
| CHAIN-n01000-ROT | 14.1 | 11.8 | 13.9 | 11.7 | Selected 128 |
| CHAIN-n10000-AX | 13.7 | 9.2 | 13.8 | 9.4 | Unresolved(ExactSumSpan) |
| CHAIN-n10000-ROT | 22.6 | 16.3 | 22.8 | 16.3 | Unresolved(ExactSumSpan) |
| CONT-n00010-AX | 8.1 | 6.5 | 7.5 | 5.9 | Selected 128 |
| CONT-n00010-ROT | 10.7 | 8.9 | 10.4 | 8.7 | Selected 128 |
| CONT-n00100-AX | 7.3 | 5.8 | 8.1 | 6.5 | Selected 128 |
| CONT-n00100-ROT | 9.9 | 8.5 | 10.0 | 8.4 | Selected 128 |
| CONT-n01000-AX | 7.5 | 5.9 | 7.5 | 6.0 | Selected 128 |
| CONT-n01000-ROT | 10.4 | 8.8 | 10.5 | 8.9 | Selected 128 |
| CONT-n10000-AX | 7.6 | 6.0 | 7.6 | 6.1 | Selected 128 |
| CONT-n10000-ROT | 7.9 | 6.5 | 7.8 | 6.5 | Unresolved(ExactSumSpan) |
| TREE-n00010-AX | 9.0 | 7.1 | 9.0 | 7.2 | Selected 128 |
| TREE-n00010-ROT | 10.6 | 9.2 | 10.7 | 9.2 | Selected 128 |
| TREE-n00100-AX | 9.5 | 7.6 | 8.9 | 7.2 | Selected 128 |
| TREE-n00100-ROT | 11.3 | 9.8 | 11.4 | 9.0 | Selected 128 |
| TREE-n01000-AX | 9.5 | 7.7 | 9.6 | 7.7 | Selected 128 |
| TREE-n01000-ROT | 11.8 | 10.0 | 11.8 | 9.9 | Selected 128 |
| TREE-n10000-AX | 14.1 | 9.8 | 14.2 | 9.7 | Unresolved(ExactSumSpan) |
| TREE-n10000-ROT | 20.7 | 15.1 | 20.8 | 14.9 | Unresolved(ExactSumSpan) |
| DEC053 cantilever-chain-24 | 8.0 | 6.4 | 7.5 | 5.9 | Selected 128 |
| DEC053 cantilever-chain-32 | 7.7 | 6.1 | 7.7 | 6.2 | Selected 128 |
| DEC053 cantilever-chain-48 | 7.6 | 6.0 | 7.5 | 6.0 | Selected 128 |
| DEC053 cantilever-chain-8 | 7.3 | 5.8 | 7.8 | 6.2 | Selected 128 |
| DEC053 grid-frame-4x3 | 7.6 | 6.1 | 7.5 | 6.1 | Selected 128 |
| DEC053 grid-frame-5x5 | 9.6 | 7.8 | 10.1 | 8.2 | Selected 128 |
| DEC053 grid-frame-5x6 | 10.4 | 8.5 | 10.4 | 8.4 | Selected 128 |
| DEC053 grid-frame-6x8 | 11.7 | 9.5 | 11.6 | 9.5 | Selected 128 |
| DEC053 grid-frame-7x8 | 12.5 | 10.2 | 12.2 | 9.6 | Selected 128 |

## T6. Growth fits (the packet: log-log least squares against members, both passes, classification ok)

| family/mode | heap-move slope | points | net-RSS slope |
|---|---|---|---|
| CHAIN/sparse | 0.999 | 16 | 0.816 |
| CHAIN/w1a | 0.948 | 16 | 0.812 |
| CONT/sparse | 1.002 | 16 | 0.778 |
| CONT/w1a | 0.919 | 16 | 0.793 |
| DEC053/sparse | 0.873 | 18 | 0.365 |
| DEC053/w1a | 0.881 | 18 | 0.572 |
| TREE/sparse | 0.998 | 16 | 0.822 |
| TREE/w1a | 0.979 | 16 | 0.824 |

## T7. rho and heap/E by size (b3, pass 1)

| mode | size | n | rho_fp min–max | heap/E min–max | largest rho_fp |
|---|---|---|---|---|---|
| sparse | 10 | 6 | 2.729–3.058 | 0.639–0.725 | TREE-n00010-ROT |
| sparse | 100 | 6 | 0.995–1.594 | 0.586–0.768 | CONT-n00100-AX |
| sparse | 1000 | 6 | 0.846–1.271 | 0.667–0.796 | TREE-n01000-AX |
| sparse | 10000 | 6 | 0.721–1.753 | 0.642–0.731 | CHAIN-n10000-AX |
| sparse | DEC053 | 9 | 1.117–3.224 | 0.594–0.724 | DEC053 cantilever-chain-8 |
| w1a | 10 | 6 | 0.105–0.200 | 0.042–0.062 | CONT-n00010-AX |
| w1a | 100 | 6 | 0.264–0.393 | 0.198–0.260 | CONT-n00100-AX |
| w1a | 1000 | 6 | 0.385–0.432 | 0.288–0.297 | TREE-n01000-ROT |
| w1a | 10000 | 6 | 0.316–0.375 | 0.305–0.307 | CONT-n10000-AX |
| w1a | DEC053 | 9 | 0.104–0.317 | 0.045–0.186 | DEC053 cantilever-chain-24 |

## T8. The sparse runs at 10,000 members (b3; K6's own mode, for the record)

| model | pass | class | heap MiB | footprint MiB | RSS MiB | E_adm MiB | heap/E | rho_fp |
|---|---|---|---|---|---|---|---|---|
| CHAIN-n10000-AX | 1 | NumericallyUnresolved | 254.2 | 693.2 | 717.0 | 394.8 | 0.644 | 1.753 |
| CHAIN-n10000-AX | 2 | NumericallyUnresolved | 254.2 | 687.9 | 711.7 | 394.8 | 0.644 | 1.740 |
| CHAIN-n10000-ROT | 1 | NumericallyUnresolved | 277.1 | 285.8 | 340.7 | 394.8 | 0.702 | 0.721 |
| CHAIN-n10000-ROT | 2 | NumericallyUnresolved | 277.1 | 284.9 | 334.7 | 394.8 | 0.702 | 0.719 |
| TREE-n10000-AX | 1 | NumericallyUnresolved | 258.9 | 391.0 | 416.2 | 394.8 | 0.659 | 0.988 |
| TREE-n10000-AX | 2 | NumericallyUnresolved | 258.9 | 391.0 | 416.2 | 394.8 | 0.659 | 0.988 |
| TREE-n10000-ROT | 1 | NumericallyUnresolved | 286.2 | 570.4 | 667.9 | 394.8 | 0.731 | 1.442 |
| TREE-n10000-ROT | 2 | NumericallyUnresolved | 286.2 | 570.3 | 666.3 | 394.8 | 0.731 | 1.442 |
| CONT-n10000-AX | 1 | Sensitive | 243.4 | 341.0 | 365.7 | 379.4 | 0.642 | 0.896 |
| CONT-n10000-AX | 2 | Sensitive | 243.4 | 341.5 | 366.2 | 379.4 | 0.642 | 0.897 |
| CONT-n10000-ROT | 1 | Sensitive | 267.4 | 295.3 | 380.2 | 379.4 | 0.705 | 0.776 |
| CONT-n10000-ROT | 2 | Sensitive | 267.4 | 332.9 | 389.8 | 379.4 | 0.705 | 0.875 |
