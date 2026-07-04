# Reborn Readme

## So what is Reborn?

Reborn is a game engine written in Rust and based around
state machines. Think like Godot or Bevy, only instead of
having each entity storing data, it might store a state or
collection of states, and these states (being Rust enums)
can store data (currently support is limited to `i64`s).

That's not actually what it is, though. It's a game engine
I made in the span of a single day on 5 hours of sleep, with
quality to match. My original idea was to make it in one week,
but after some OS issues, I had to switch OSes and forgot to
back up my data, so I decided I'd just redo all the progress
in a single day.

I didn't succeed at all the plans, I had to cut off graphics
and make it entierly text based, but I hope to use graphics
later. Still, I succeeded at the actual goal, which was to
push myself and learn.

## So why should I use it?

You shouldn't. It's extremely unstable and basically incapible
of making any real games. I made it entierly as an intellecutual
excercise. However, the entier engine is only 42 lines if you
want to inspect the internals or contribute.

In the future, if I keep developing it (big if), you'll want
to use it because it makes keeping track of state and implementing
behavior based on state trivial. That's the goal.

## Goals

**Long term:**

* Parralel processing of entities
* Physics handled by a library or on the GPU with a custom implementation
* Full 3D and 2D
* More erganomic state handeling
* Visual level editor

But let's be real, it probably won't ever get there unless other people show
interest. The point of the engine was almost entierly to make an engine in a
few days, and it's already served that purpouse (and ideally to show state
based entities work, which it also did).

**Short term:**

* Renders 2D squares
* Handles input
* Basic AABB collision

I'll work on it another day or two, probably. Maybe. I want to at least have
a semicapable game engine.

### Philosophy

The goal is to get out of the way of the dev. Native state machines to automate state
related work, no editor forcing GUI bloat on them (except for level design), no bloated
tools that do a thousand things that should be done by different tools. Working is
better than perfect. Basically, don't make it miserable to use.

## Contributing

**Basic Rules:**

* Be civil. It's ok to disagree, but be respectful about it.
* Don't be a jerk. No racism, homophobia, transphobia, sexism, etc.
* Accept feedback. If someone takes effort to tell you how to improve something,
you should thank them instead of getting offended.
* Just be decent, ok?
* Make sure your code is aligned with the philosophy.

**Code Standards:**

* Nothing dependent on Windows or macOS. This engine is Linux-first,but ideally aims
to support everyone.
* NO propietary dependancies.
* It needs to run.

**AI Use Policy:**

* The entire point of this project is to enhance your understanding of code and game
engines. ONLY use AI to enhance your understanding, not to replace it.
* In more clear terms, no "vibe coding". You can use AI as a supplemental resource to
help you understand things you're having trouble understanding, tell you why your code
won't compile, and *maybe* automate truly repetative tasks. Don't use it to do actual
thinking, because that defeats the entier point of this project.

If you can follow those rules, just open a PR and I'll be happy to inspect it!
