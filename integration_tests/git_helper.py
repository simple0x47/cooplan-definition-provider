import git
import os

# Repository has the following structure: user/repository
DEFINITION_GITHUB_USER_REPOSITORY_ENV = "DEFINITION_REPOSITORY"
DEFINITION_GIT_USER_ENV = "DEFINITION_GIT_USER"
DEFINITION_GIT_TOKEN_ENV = "DEFINITION_GIT_TOKEN"


def get_latest_definition_version(local_path: str):
    repository = os.environ.get(DEFINITION_GITHUB_USER_REPOSITORY_ENV)

    if repository is None:
        print(f"environment variable {DEFINITION_GITHUB_USER_REPOSITORY_ENV} not set")
        return None

    git_user = os.environ.get(DEFINITION_GIT_USER_ENV)

    if git_user is None:
        print(f"environment variable {DEFINITION_GIT_USER_ENV} not set")
        return None

    git_token = os.environ.get(DEFINITION_GIT_TOKEN_ENV)

    if git_token is None:
        print(f"environment variable {DEFINITION_GIT_TOKEN_ENV} not set")
        return None

    repository_url = f"https://{git_user}:{git_token}@github.com/{repository}.git"

    repository = git.repo.Repo.clone_from(repository_url, local_path)

    return repository.head.object.hexsha
