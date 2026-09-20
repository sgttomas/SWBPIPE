# Exact CUA calls for the f5 native witness

Target binding:

```javascript
let finalApp = await cua.getApp("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-wt3/projects/chirality-piping/apps/desktop/src-tauri/target/debug/bundle/macos/SWBPIPE.app");
```

Model history preparation used the visible Node control and settable inputs:

```javascript
await finalApp.click(100);
await finalApp.setValue(135,'node:N-WITNESS');
await finalApp.setValue(138,'Witness node');
await finalApp.setValue(141,'0');
await finalApp.setValue(144,'0');
await finalApp.setValue(147,'0');
await finalApp.setValue(154,'invented_native_witness');
await finalApp.click(155);
await finalApp.click(162);
```

Model undo/redo used actual native key input after moving to the visible Select
control:

```javascript
await finalApp.click(13);
await finalApp.pressKey('super+z');
await finalApp.pressKey('super+shift+z');
```

The focused-text check used both CUA text input and a separate single-key input:

```javascript
await finalApp.click(50);
await finalApp.typeText('Witness');
await finalApp.pressKey('super+z');
await finalApp.pressKey('x');
await finalApp.pressKey('super+z');
await finalApp.getAXState({disableDiffing:true});
```

The first `super+z` left `Witness`; the second left `Witnessx`. The full AX
snapshot after the second showed the model node still selected and model Undo
still enabled.

HTML select cancellation:

```javascript
await finalApp.click(155);
await finalApp.getAXState();
let popupShot = await finalApp.getScreenshot({emit:false});
await finalApp.pressKey('Escape');
await finalApp.getAXState({disableDiffing:true});
```

The pre-Escape AX result was a menu with selected `m` and `mm`, `in`, `ft`.
The post-Escape full tree had no popup and had Inspector collapsed.

Debug inspector/dimension attempts, made once each:

```javascript
await finalApp.click([700,400], {mouseButton:'right'});
await finalApp.getAXState();
await finalApp.pressKey('super+alt+i');
await finalApp.getAXState();
await finalApp.click(165);
await finalApp.getAXState();
```

The context click and shortcut produced no accessibility change. The SWBPIPE
app menu showed only About and Quit entries. No further inspector attempt was
made.
