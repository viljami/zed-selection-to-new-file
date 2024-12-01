# Move Selection To A New File

This is the [ZED Editor](https://zed.dev/) [Extension](https://zed.dev/docs/extensions/developing-extensions), designed to move selected text from the current file to a new, unnamed file. Often, when ideas are still vague, we write features or improvements in the existing file. As we implement code, our understanding becomes clearer, revealing the need for modularization. This extension aids in creating modules as part of your natural coding process:

* Make it Work: Transition from vague ideas to clear implementations.
* Make it Pretty: Refactor your code from functional to readable modules.
* Make it Fast: Optimize performance with a clearer structure.

To split the code into modules, follow these simple steps:

1. Select the text in the editor.
2. Press the key combination to activate the extension, which will:
    1. Cut the selected text from the current file.
    2. Create a new file.
    3. Paste the selected text into the new file.
3. Save the new file and assign it a name.

Join us in enhancing coding efficiency and organization!

## Implementation approaches

1. Custom keymap (not working)
2. Task & custom keymap
2. Extension

### Custom Keymap

With `workspace::SendKeystrokes` we can chain commands to be triggered with a
custom key combination `cmd-x cmd-n cmd-v`.

1. `cmd-x` cut text
2. `cmd-n` new file
3. `cmd-v` paste text

The problem here is that an asynchronous operation is completed after the text is
pasted. Which leads the text being pasted to the original file like no cut operation
was done in the first place and the new file resulting empty.

Implementation for `keymap.json` file:

```json
[
  {
    "context": "Workspace",
    "bindings": {
      "ctrl-cmd-x": ["workspace::SendKeystrokes", "cmd-x cmd-n cmd-v"]
    }
  }
]
```

### Task & Custom Keymap

Not implemented.

### Extension

Meant for Assistant Panel which already have great context filling commands.
Might not be the right approach.

[Stub only](./src/lib.rs)
