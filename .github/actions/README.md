# GitHub Actions

Shared actions and utilities for zed-stylelint CI/CD.

## Structure

- `shared/` - Shared utilities used by multiple actions
- Each action has its own directory with `action.yml` and `index.ts`

## Development

1. Install dependencies:
   ```bash
   cd .github/actions
   npm install
   ```

2. Type check actions:
   ```bash
   npm run typecheck
   ```

## Adding a New Action

1. Create directory: `mkdir my-action`
2. Create `action.yml` with inputs/outputs (point to `index.ts`)
3. Create `index.ts` with main logic
4. Import from `../shared/src/` for utilities
5. Run type check before committing: `npm run typecheck`

## Testing

Actions can be tested locally using [act](https://github.com/nektos/act):

```bash
act -j check-and-update
```
