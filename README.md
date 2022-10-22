# Ruststone

Ruststone is an implementation of Minecraft's Redstone logic. It will be able to emulate the Redstone logic as it exists in Java Edition (the real edition).

It'll also, eventually, offer a programming language that compiles down to Redstone.

## But, why?

Quite frankly, this problem was just too irresistible. I thought about making a programming language that could trivially be transformed into Redstone logic. I figured, to be able to mechanistically verify that such logic works, I need to implement an emulation of Redstone. From there, I can then build a language atop Minecraft Redstone.

I also hope there will be some very interesting takeaways that I can use to build other things in the future.

## Things we want

### Redstone emulation

An emulation of Redstone logic as seen in Minecraft, Java Edition.

### The Programming Language (name TBD)

A programming language that can be compiled down into Redstone logic.

The starting point will be very simple: only boolean logic is allowed. I don't currently have the full understanding of the limitations of Redstone logic, so that means we're not going to try implementing numbers, strings, or modern programming features e.g. higher order functions or closures. Maybe someday.
