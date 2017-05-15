#### mush

mush is currently targeting [lichen](https://github.com/viperscape/lichen) for its dialogue graph backend, but ultimately will be generic and could target anything that implements some basic graph traits. The purpose of mush is to provide a clean and easy front end with visual representation of dialogue graphs.

Ideally this means a live step-debugger and modifiers to nodes and their statements/data (think Unreal blueprints during runtime). Since lichen is not just a dialogue tree but more of a miniature scripting language, it has features that would likely be missing from basic dialogue graph solutions.

These include advanced logic operations, state mutations, and internal variable types. mush will be designed around these concepts and so implementing a basic dialogue tree outside of lichen might be less advantageous.

![file-opener](/file-opener.png)  


![parse-env](/parsed-env.png)  
