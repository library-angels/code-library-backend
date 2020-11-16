if __name__ == "__main__":
    import json
    import sys

    import requests

    GITHUB_API = "https://api.github.com"
    REPO = "library-angels/code-library-backend"
    API_URL = f"{GITHUB_API}/repos/{REPO}/commits"

    # list of commit shas of pull request
    commit_list = json.loads(sys.argv[1])
    # we want to the detect if these files changed
    relevant_files = json.loads(sys.argv[2])

    # get all changed files (added, modified, or removed)
    file_list = set()
    for sha in commit_list:
        for file_obj in requests.get(f"{API_URL}/{sha}").json()["files"]:
            file_list.add(file_obj["filename"])

    # extract first path segment for all files, w/o duplicates
    base_paths: set = {file.split("/")[0] for file in file_list}

    # get full path to gh-actions workflows
    if ".github" in base_paths:
        for file in file_list:
            if file.startswith(".github/workflows"):
                base_paths.add(file)
        base_paths.remove(".github")

    # did any relevant file change?
    relevant_file_changed = True in [path in base_paths for path in relevant_files]

    # output result to stdout
    print(relevant_file_changed)
