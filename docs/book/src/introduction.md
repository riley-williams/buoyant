# Introduction

Buoyant makes it easy to create flexible, dynamic, and (eventually) interactive UIs
on embedded systems. It is designed to be used with the `embedded-graphics` crate, but
can be adapted to other rendering targets.

The features and API language are heavily influenced by SwiftUI. If you're already familiar
with SwiftUI, you should feel right at home with Buoyant. If you aren't, don't worry.

## Why create this?

The vast majority of my frontend experience is with SwiftUI, and I just wanted something
like that for embedded Rust.

The well known `std` Rust UI crates rely heavily on dynamic allocation making them unsuitable
for porting to embedded. On the embedded side, at least as of the time of writing, there are
no other good solutions for creating UIs with embedded Rust. You're essentially stuck manually
placing elements with ``embedded_graphics``, which is both incredibly tedious and not at all
scalable. If you want animation, you're in for a world of pain.

This is my attempt to fill that need, and at least so far, it's been far more successful than
I imagined. While Buoyant is still very young and I'm still learning a lot about Rust, it's
already capable of building fairly complex UIs in a scalable declarative way. Animations are
incredibly easy to add, and it's possible to actually feel productive.
