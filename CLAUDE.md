- In all interactions and commit messages, be extremely concise and sacrifice grammar for brevity.

Git workflow:
- Work in feature branch
- Commit messages: imperative present tense ("Move implementation", "Abstract with a macro", "Avoid specifying defaults")
- *Atomic commits* - minimal, focused changes for easy reordering/squashing
- Build & test after every change
- Always use `just` commands, e.g. `just build`, `just check`, `just fmt`, not `cargo ...`
- Use `cd` and git without `-C` option (maintain working directory)

Look for ways to parallelise work and delegate to subagents. Spawn subagents for isolated, well-defined tasks of medium size.
