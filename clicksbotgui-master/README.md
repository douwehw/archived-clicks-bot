# Clicksbot GUI

This is a GUI version of [ClicksbotX](https://github.com/paroxysms/clicksbotX).

Version 1.2 of Alphas Clicksbot, the popular Geometry Dash cheat.

The program is currently in development, but this will most probably the last early access build.

# Build arguments

To build the .exe run `cargo rustc --bin clicksbotgui --release -- -C link-args="/ENTRY:startup /SUBSYSTEM:console /DYNAMICBASE:NO /FIXED /NXCOMPAT:NO /SECTION:.code,ERW /SECTION:.stub,ERW"`
