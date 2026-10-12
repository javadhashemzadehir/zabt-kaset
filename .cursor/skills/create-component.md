# Skill: create-component

Create a React component in the desktop app.

## Steps

1. Decide location:
   - Generic UI → `apps/desktop/components/ui/`
   - Layout → `apps/desktop/components/layout/`
   - Domain → `apps/desktop/features/<domain>/`
2. Use TypeScript + function components.
3. Style with Tailwind tokens from `globals.css`.
4. Export a named function (no default for UI primitives unless shadcn style).
5. Add loading/empty/error handling if the component fetches data.
6. Ensure RTL-safe spacing classes.
