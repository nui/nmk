$BRANCH = git for-each-ref --format='%(refname:short)' $(git symbolic-ref -q HEAD)
$TRACKED_BRANCH = git for-each-ref --format='%(upstream:short)' $(git symbolic-ref -q HEAD)

if ($TRACKED_BRANCH.Length -gt 0) {
    git remote update --prune
    git reset --hard $TRACKED_BRANCH
}
else {
    echo "$BRANCH is not tracked any remote branch"
}
