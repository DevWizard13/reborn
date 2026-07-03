# Concept

The goal of this document is to write down my ideas for the engine as it proceeds
through the week I develop it.

## Goals

The goal of this game engine is to A: prove that a game engine can be built in a
week, B: prove *I* can make a game engine in a week, and C: get out of the dev's way
without forcing bloated GUIs on them (looking at you, Unity/UE/Godot) or sitting in
development for almost a decade and still lacking fundamental features (looking at
you, Bevy) so that they can make games.

A game engine is a software designed for use making video games. Therefore, we should
look at how games run. Fundamentally, it's all about state. Is a player running,
attacking, jumping, or falling? The game itself is a state too, have you started?
Are you in the middle of it, or have you finished?

## Implementation

So, anyway, that's the fundamental goal. The engine will be based around entities,
which can have a number of state machines. Each state machine has a method called
`get_state() -> String`, which returns a string repesenting the state. Other methods,
such as `get_state_val(state: String) -> Optional<f64>` will also be present in the
event certain states have values, such as a health state or a position state.

For example, a health state may look like:

```
enum Health {
  ALIVE(i32),
  DEAD,
}
```

each entity will have a set of these state machines attached to them that determine
their behavior. For example, a player may have a movement component, a position
component, a health component, and a weapon component. These all have their own
states and can affect each other. Systems will be able to operate on sets of
components, operating on them per-state. For example, a system may operate on all
entities that have both a position and movement component, and update the position
based on the state of the movement component (falling, running, idle, jumping).
