# bevy_renderdoc_capture

This is a simple [Bevy](https://bevyengine.org/) module to add a custom keybind for doing RenderDoc captures around Bevy's renderer. Why would you need this? When I was messing with VR stuff, RenderDoc couldn't figure out the frame boundaries itself.

To use:

1. Add to your project.
2. Add the `RenderdocPlugin` plugin to your app.
3. Run your game in RenderDoc.
4. Press F10.

