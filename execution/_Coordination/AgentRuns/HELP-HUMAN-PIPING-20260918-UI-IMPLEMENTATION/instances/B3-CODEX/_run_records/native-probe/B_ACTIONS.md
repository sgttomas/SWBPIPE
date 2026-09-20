# Variant B actual CUA sequence
Same exact probe bundle path, fresh PID13719. Devtools Close click3; File click651; Open click5. Project remained exact disposable blank row, no intervening store writes in A.
Node click100. setValue135 node:N-PROBE, 138 Probe node, 141/144/147 0, 154 invented_native_probe. click155 Add; AX validation passed (base hash sha256:6de0261054a94e3263a90ad5b3c54b7ab684c28eca3f4fe54c916193be2c2143); click162 Apply. One node/model checkpoint. AX setValue preparation is distinct from later per-key text history and is not asserted to establish an empty native stack.
click13 Select; super+z -> one node removed, Undo disabled/Redo enabled. super+shift+z -> one node restored, Undo enabled/Redo disabled. These precede filter per-key text stack population.
click50 Filter; pressKey x,y,BackSpace -> x. super+z -> empty; super+shift+z -> x. Model checkpoint unchanged.
click13 Select; super+z -> node removed; super+shift+z -> node restored, text remains x. click50; super+z -> empty, proves native text undo history survived model commands.
click13; super+z -> model removed; super+shift+z -> model restored, text remains empty. click50; super+shift+z -> x, proves native text redo history survived model commands.
click13; click123 Edit -> menu has separate Undo Model Edit and native Undo undo:/Redo redo:. click3 Undo Model Edit -> node removed. click124 Edit -> custom Undo disabled/Redo enabled. click4 Redo Model Edit -> node restored. Final screenshot and full AX retained.
Each action group followed by getAXState before adaptive next action; returned AX bytes saved for B tests. Some files contain AX diff rather than full tree, so read in sequence with final full AX.
