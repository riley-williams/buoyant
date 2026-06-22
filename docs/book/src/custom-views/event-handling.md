# Event Handling

## `KeyDown` and `KeyUp`

Key events should be routed to the currently focused element. Containers proxy the event to
the currently focused child only and return `Deferred` if no child is focused. Containers
should generally not mutate the focus tree or search for a handler.

## `Touch`

Touch events should be routed with depth first search. Non-matching elements should
return `Deferred`. Elements can signal to the `focus_touches()` modifier that they
should acquire focus by returning `EventResult::handled_focused()`. Elements like Button
should return `EventResult::handled_unfocused()` on touch down to indicate the touch
was handled but focus should not be moved.

## `Focus(_)`

Obtain the current element, or the next one in the specified direction if the current
element is non-matching.

Leaf views matching the requested role should return `EventResult::handled_focused()`,
otherwise `Deferred`.

Containers should search, starting with the currently focused child, until a child doesn't
return `Deferred`. If the container's children are exhausted, it should also return `Deferred`.

### `Next` and `Previous`

Obtain the next or previous matching element.

Leaf views should always return `Deferred`. A leaf only receives `Next` if it is the
currently focused element.

Containers should search, starting with the currently focused child, until a child doesn't
return `Deferred`. If the container's children are exhausted, it should also return `Deferred`.

Containers receiving `Next` and `Previous` first pass the event to the currently focused child.
If the child returns `Deferred`, the container initializes the default focus of the next
child. The event is exchanged for the corresponding `Focus(Forward/Backward)` so that the
newly-initialized child can attempt to accept focus. Without this exchange, the first
element in the new child subtree would never be able to accept focus.

### `Select`

Perform a primary action.

Leaf views matching the requested role should return `EventResult::handled_focused()`,
otherwise `Deferred`.

Containers should proxy the event to the currently focused child, but should not mutate
the focus tree nor attempt to locate a previously unfocused child which handles the event.

### `Blur`

Most elements should simply pass the event to the currently focused child and return the result.

Elements which can capture focus should first pass the event to their currently focused
child. If the child responds with `Deferred`, the captive element should release focus and
return `EventResult::handled_focused()`. If the child indicates the event was handled,
captive focus should not be released and the child result returned.

### `Teardown`

Secondary state that tracked an element as being focused should be reset. A stale focus tree
is about to be dropped, e.g. as a result of a touch causing focus to jump.

`Teardown` should not normally need to be called. It should be assumed that a `Focus(_)` event
will follow, reasserting focus, if the element is still focused in the new tree. I think
this (and its associated weirdness) can be removed by adding an `Option<&mut Self::FocusTree>`
arg to `layout` and `render_tree`. This would alleviate the pesky need to track any extra
non-source-of-truth state.

Containers must not wrap navigation on `Teardown`: they proxy the event to the currently
focused child and return the result without looping.
