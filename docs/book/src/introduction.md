# Introduction

Buoyant makes it easy to create flexible, dynamic, and (eventually) interactive UIs
on embedded systems. It is designed to be used with the `embedded-graphics` crate, but
can be adapted to other rendering targets.

The features and API language are heavily influenced by SwiftUI. If you're already familiar
with SwiftUI, you should feel right at home with Buoyant. If you aren't, don't worry.

## Why create this?

The vast majority of my frontend experience is with SwiftUI, and I just want to use it
for embedded. Despite what Apple would like you to think, Swift isn't all that great for
embedded, so here we are doing it in Rust.

The well known `std` Rust UI crates rely heavily on dynamic allocation, making them unsuitable
for porting to embedded.

On the embedded side, at least as of the time of writing, there weren't any other solutions
I found very satisfying. I'm not really interested in buying into the Slint ecosystem,
and aside from that, you'd essentially be stuck manually placing elements with
``embedded-graphics``. Not fun at all.

This is my attempt to fill that need, and at least so far, it's been far more successful than
I imagined. While Buoyant is still very young and I still feel new to Rust, Buoyant is
already capable of building fairly complex UIs in SwiftUI's declarative style. Animations are
not only possible, but quite easy to add.
