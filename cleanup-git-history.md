# Squashing Commits to Remove API Keys from Git History

We've identified API keys in several example files in the git history. Since adding `.env` to `.gitignore` only prevents future commits from including sensitive information (but doesn't remove existing keys from history), we'll use git squash to create a clean history.

## Step-by-Step Squash Process

Here's how to squash commits to eliminate API keys from the git history:

```bash
# 1. First, make sure all your changes are committed
git status

# 2. Create a backup branch just in case
git branch backup-before-squash

# 3. Identify how many commits are in your history
git log --oneline

# 4. Create a new orphan branch (this has no history)
git checkout --orphan temp-clean-branch

# 5. Add all files from the current state
git add .

# 6. Commit with a clean message
git commit -m "Initial commit with API keys removed and .env support added"

# 7. Rename branches (assuming you're working on main/master)
git branch -D main        # Delete the old main branch
git branch -m main        # Rename current branch to main

# 8. Force push to update the remote
git push -f origin main
```

This approach creates a completely new history with just one commit, effectively removing all traces of the API keys from the git history.

## After Squashing: Important Security Steps

After squashing commits to remove API keys from git history, take these additional security steps:

1. **Revoke all exposed API keys immediately**
   - Even though they're removed from git history, consider them compromised
   - Generate new API keys through the Venice.ai dashboard
   - Update your .env file with the new keys

2. **Notify team members**
   - Let all collaborators know about the force push
   - They'll need to re-clone or reset their local repositories
   - Share the new .env template (without actual keys) with the team

3. **Verify the cleanup**
   - Check that API keys are no longer in the git history:
     ```bash
     git log -p | grep -E "[A-Za-z0-9_-]{30,}"
     ```
   - Confirm .env is properly ignored:
     ```bash
     git check-ignore -v .env
     ```

## Preventing Future API Key Leaks

Now that we've implemented .env for API key management and cleaned the git history, follow these best practices:

1. **Always use environment variables or .env files**
   - Never hardcode API keys in source code
   - Use our new dotenv_example.rs as a reference

2. **Set up git hooks to prevent committing sensitive data**
   - Install git-secrets: https://github.com/awslabs/git-secrets
   - Configure patterns to block API key formats:
     ```bash
     git secrets --add '[A-Za-z0-9_-]{30,}'
     ```

3. **Use our debug_headers.rs example for API debugging**
   - It masks API keys in the output
   - Provides safe debugging of API requests

4. **Consider a secrets management solution**
   - For production: HashiCorp Vault, AWS Secrets Manager, etc.
   - For CI/CD: Use environment variables in your CI system

5. **Regular security audits**
   - Periodically scan your codebase for accidental API key commits
   - Rotate API keys regularly as a best practice

By following these steps, you've successfully removed API keys from git history and implemented a secure approach to API key management using .env files and proper gitignore settings.