# GitHub Copilot Instructions

## GitHub Actions Workflows

- **Always use explicit version tags** for actions instead of branch references (e.g., `@main`, `@master`)
- Example: Use `uses: actions/checkout@v4` instead of `uses: actions/checkout@main`
- This ensures reproducible builds and prevents unexpected breaking changes
