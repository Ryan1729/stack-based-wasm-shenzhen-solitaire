# What does stack-based mean?

Specifically, what does stack-based mean in the context of this project? To answer that we need to talk about what the purpose of this project is. I'm doing this because I want to know more about the best way of expressing solitaire games in stack-based bytecode. The reason I want to know more about expressing solitaire games is because I'm working on a project which currently seems like a [solitaire game generator](https://github.com/Ryan1729/evolving-games). The reason I am wanting to use a stack-based VM is that [this book chapter](http://gameprogrammingpatterns.com/bytecode.html) convinced me that it was a good fit for the problem of a data format for expressing behavior. Since this project is for practicing doing things within constraints, it seems prudent to set hard constraints for it. 

The restrictions that seem to fit best with the stated goal are as follows:

* All the `update` code should be written in a purely stack based style, (more on this later.)
* All of the "Shenzhen-specific" stuff should be expressed within the either the bytecode or another auxiliary data format.

## Okay, but again, "What does stack-based mean?"

All of the code will need to be expressed as instructions for a virtual machine. All of the working memory for that VM will be stored on a single stack. [The previously mentioned book chapter](http://gameprogrammingpatterns.com/bytecode.html) makes a distinction between two different types of VM, stack-based and register-based. Essentially the difference is that stack-based VMs are restricted to `push` and `pop` where register based VMs would have a `peek` operation which lets them look at values further down the stack. The chapter suggests that stack-based VMs are easier to generate code for, and since that is our eventual goal, it seems like a good idea to go with that. If we find out that we really need the extra power of a register-based VM, (a breif search seems to suggest that the lack of random memory access makes stack-based VMs not Turing-complete,) then that's fine, the point of this project is to learn. But we should give it an honest go without that power first.

# Implementation plan

Part of the reason that "porting" this game to start generating games by generating rust source code didn't go so well is because it needed to be done all at once. Hopefully we can avoid that problem by starting small and maintaining a working game most of the time. The goal is to find/design a bytecode that is powerfull enough to express many different solitaire style games without just implementing a general purpose language, (which, besides taking a while and likely producing worse usability compared to something worked on for longer by more people, it would also balloon the search space of programs too much.) So the approach that seems best is to translate the top level functions below `update` into bytecode directly, then slowly make more of the game programmable/customizable without going too far down that rabbit hole. That said, we can probably assume the rough structure of the top level of the update function can stay the same. Specifically the structure of a move timer and an `automove` procedure, along with the input processing and behavior after a win. In fact the input bytecode should probably consist of separate sets of instructions for the following portions of the game: 

* initialization
* automove
* separate sets for each button
* win checking

____

# moving enough into bytecode that we can start generating

It's unclear how easy generating bytecode that maintains invariants like winnability or crash avoidance will be. Therefore we want to limit the amount of time spent on the bytecode before we attempt to generate it. At the same time we don't want generating to fail only because there's no interesting programs to generate. So we want to move just enough into bytecode that at least one interesting thing can be generated, then start generating.

So first we need to decide which game we want to try and implement in bytecode. What would we need in order to implement Klondike, admittedly a similar game to Shenzhen Solitaire, or at least something close enough to it? The most obvious difference is the deck. We could, of course, add a way to specify the contents of the deck as part of the bytecode or in some adjacent data. But then we would need to add more graphics. While we could do that, the set of games that naturally translate into a standard deck of playing cards has been explored pretty throughly. I'd like to at least attempt to find an interesting distinct game which uses the Shenzhen deck verbatim. One simple but potentially distinct version would be to use the flower foundation as a regular foundation and just not care where the flower ends up. We'll say that the flower can be stacked on anything but nothing can be stacked on it. Is that sufficiently distinct? It is at least distinct enough that if a generator generated that I would be at least somewhat impressed. We can always expand the bytecode space later if we are actually able to exhaust, (or exhaust interest in,) a given space.

What would be required to be part of the bytecode to allow that? All the foundation pile checking which includes more or less all of the update functions except for `movecards`. Since we would need to make the `canmovedragons` part of the bytecode we would need to have a way for the bytecode to indicate which buttons can be pushed since the drawing code needs to know that. That is, assuming we want to allow moving dragons to the flower foundation. If we don't want to do that, then we can postpone that for later, but since it would increase the amount of possible games greatly, (three buttons that can run bytecode under arbitrary bytecode-checkable conditions!), that would be a good area to expand the bytecode into eventually.

In summary, we should convert everything called from the input if-else block in `update`, including `automove` except for `canmovedragons` and `movedragons`.
