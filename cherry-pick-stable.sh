#!/bin/bash

set -euo pipefail

# Parse arguments
DRY_RUN=false
while [[ $# -gt 0 ]]; do
    case $1 in
        --dry-run|-d)
            DRY_RUN=true
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Cherry-pick commits from PRs labeled 'stable-nominated' to current branch"
            echo ""
            echo "Options:"
            echo "  -d, --dry-run    Show what would be done without making changes"
            echo "  -h, --help       Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

if [ "$DRY_RUN" = true ]; then
    echo "[DRY RUN MODE - No changes will be made]"
    echo ""
fi

current_branch=$(git branch --show-current)
echo "Current branch: $current_branch"
echo "Fetching PRs with 'stable-nominated' label..."
echo ""

# We need to use the graphql API to be able to return more than ~40 PRs.
# `--paginate` with `--slurp` will repeat as needed to collect all items.
# shellcheck disable=SC2016
input=$(gh api graphql --paginate --slurp -f query='
    query ($endCursor: String) {
      repository(name: "libc", owner: "rust-lang") {
        pullRequests(
          first: 25
          after: $endCursor
          baseRefName: "main"
          states: MERGED
          labels: ["stable-nominated"]
          # Unfortunately ordering by merge date is not supported
          orderBy: {field: CREATED_AT, direction: ASC}
        ) {
          nodes {
            title
            number
            state
            url
            author {
              login
            }
            mergedAt
            mergeCommit {
              oid
              committedDate
              author {
                name
              }
            }
            commits {
              totalCount
            }
          }
          pageInfo {
            hasNextPage
            endCursor
          }
        }
      }
    }'
)

prs=$(echo "$input" | jq '
    # Pagination returns an array of arrays of objects. Flatten these and collect
    # only the data we are interested in.
    map(.data.repository.pullRequests.nodes) | flatten
    # Oldest to newest
    | sort_by(.mergedAt)
    '
)

if [ -z "$prs" ]; then
    echo "No PRs found with 'stable-nominated' label."
    exit 0
fi

# Arrays to track results
declare -a successful
declare -a unneeded
declare -a skipped

pr_count=$(echo "$prs" | jq -r 'length' )

echo "Found $pr_count PRs to cherry-pick:"
echo ""

# Turn each array item into a line for execution
prs_split=$(echo "$prs" | jq -c '.[]')

while read -r pr; do
    pr_number=$(echo "$pr" | jq -r '.number')
    commit_count=$(echo "$pr" | jq -r '.commits.totalCount')
    title=$(echo "$pr" | jq -r '.title')
    url=$(echo "$pr" | jq -r '.url')

    # For the merge strategy this is the sha of the merge commit. For rebase,
    # this is the last commit applied. There is unfortunately no easy way to get
    # a list of the rebased commits, we need to count them.
    final_commit=$(echo "$pr" | jq -r '.mergeCommit.oid')

    backport_note="(backport <$url>)"
    pr_info="PR #${pr_number}: ${title}"

    echo "----------------------------------------"
    echo "$pr_info ($commit_count commits)"
    echo "Final commit: ${final_commit}"

    # Loop through each commit in the PR
    for i in $(seq 1 "$commit_count"); do
        # Start with the oldest commit from this PR
        n_back=$((commit_count - i))
        pick_sha=$(git rev-parse "$final_commit~$n_back")
        pick_sha_short="${pick_sha:0:12}"
        pick_sum=$(git log -1 "$pick_sha" --format=%s)
        pick_sum_short="${pick_sum:0:40}"

        commit_info="$pr_info  ($i/$commit_count $pick_sha_short \"$pick_sum_short\")"

        # Check if commit already exists in current branch
        if git cherry HEAD "$pick_sha" "$pick_sha~1" | grep -q '^-'; then
            echo "⏭ Already cherry-picked, skipping"
            unneeded+=("$commit_info")
            echo ""
            continue
        fi

        # Cherry-pick with -xe flags as specified
        pick_cmd=(git cherry-pick -xe "$pick_sha_short")

        if [ "$DRY_RUN" = true ]; then
            echo "⏭ Would cherry-pick with: ${pick_cmd[*]}"
            echo "⏭ Would add backport note: $backport_note"
            successful+=("$commit_info")
            continue
        fi

        if ! git cherry-pick -xe "$pick_sha"; then
            echo "Enter c to continue after manual resolution, s to skip this commit,"
            echo "or any other key to quit."
            read -rp "selection: " -n1 selection </dev/tty
            echo

            case "$selection" in
                c) git cherry-pick --continue ;;
                s) git cherry-pick --abort
                   continue ;;
                *) git cherry-pick --abort
                   exit 1 ;;
            esac
        fi

        # Add backport note before the cherry-pick note as per CONTRIBUTING.md
        current_msg=$(git log -1 --format=%B)

        # Insert backport line before "(cherry picked from commit" line
        new_msg=$(echo "$current_msg" | sed "/^(cherry picked from commit/i\\
$backport_note\\
")

        # Amend the commit with the new message
        git commit --amend -m "$new_msg"

        echo "✓ Successfully cherry-picked $pick_sha (\"$pick_sum_short\")"
        successful+=("$commit_info")
    done
    echo ""
done <<< "$prs_split"

# Print summary
echo "========================================"
if [ "$DRY_RUN" = true ]; then
    echo "SUMMARY (DRY RUN)"
else
    echo "SUMMARY"
fi
echo "========================================"
echo ""

if [ ${#successful[@]} -gt 0 ]; then
    if [ "$DRY_RUN" = true ]; then
        echo "Would cherry-pick (${#successful[@]}):"
    else
        echo "Successfully cherry-picked (${#successful[@]}):"
    fi
    for item in "${successful[@]}"; do
        echo "  ✓ $item"
    done
    echo ""
fi

if [ ${#unneeded[@]} -gt 0 ]; then
    echo "Unneeded (${#unneeded[@]}):"
    for item in "${unneeded[@]}"; do
        echo "  ⏭  $item"
    done
    echo ""
fi

if [ ${#skipped[@]} -gt 0 ]; then
    echo "Skipped (${#skipped[@]}):"
    for item in "${skipped[@]}"; do
        echo "  ⏭  $item"
    done
    echo ""
fi

if [ "$DRY_RUN" = true ]; then
    echo "Dry run complete! Run without --dry-run to apply changes."
else
    echo "All done!"
fi
