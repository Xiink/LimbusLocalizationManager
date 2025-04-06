import requests
import os
import toml
import json

from typing import TypedDict, Literal


class Localization(TypedDict):
    id: str
    version: str
    name: str
    flag: str
    icon: str
    description: str
    authors: list[str]
    url: str
    font: dict[str, str]
    format: Literal["compatible", "new"]
    localization_asset: str | None = None


def get_latest_release(repo: str, localization_asset: str | None = None) -> tuple[str, str, str, int]:
    url = f"https://api.github.com/repos/{repo}/releases/latest"
    response = requests.get(url)
    response.raise_for_status()
    content = response.json()
    
    version = content["tag_name"]
    description = content["body"]
    data_url = None
    size = None
    for asset in content["assets"]:
        if asset["name"].lower() == "readme.md":
            description = requests.get(asset["browser_download_url"]).text

        if not asset["name"].endswith(".zip"):
            continue

        if localization_asset is None:
            data_url = asset["browser_download_url"]
            size = asset["size"]

        if asset["name"].lower() == localization_asset:
            data_url = asset["browser_download_url"]
            size = asset["size"]

    if data_url is None:
        raise Exception(f"No data URL found for {repo}")

    return version, description, data_url, size


def main() -> int:
    gist_id = os.environ["GITHUB_GIST_ID"]
    gist_owner = os.environ["GITHUB_GIST_OWNER"]
    gist_token = os.environ["GITHUB_TOKEN"]

    script_dir = os.path.dirname(os.path.abspath(__file__))
    localizations_path = os.path.join(script_dir, "localizations.toml")
    with open(localizations_path, "r") as f:
        localizations = toml.load(f)

    current_contents = requests.get(
        f"https://gist.githubusercontent.com/{gist_owner}/{gist_id}/raw/localizations.json"
    ).json()

    current_localizations: dict[str, Localization] = {}
    for localization in current_contents["localizations"]:
        current_localizations[localization["id"]] = localization
    
    processed: dict[str, Localization] = {}
    for localization_id, data in localizations.items():
        try:
            localization_asset = data.get("localization_asset")
            version, description, data_url, size = get_latest_release(data["repo"], localization_asset)
        except Exception as e:
            print(f"Error getting latest release for {localization_id}: {e}")
            if current_localizations.get(localization_id) is not None:
                version = current_localizations[localization_id]["version"]
                description = current_localizations[localization_id]["description"]
                data_url = current_localizations[localization_id]["url"]
            else:
                continue

        processed[localization_id] = {
            "id": localization_id,
            "version": version,
            "name": data["name"],
            "flag": data["flag"],
            "icon": data["icon"],
            "description": description,
            "authors": data["authors"],
            "url": data_url,
            "size": size,
            "fonts": data["fonts"],
            "format": data["format"],
        }

    headers = {
        "Authorization": f"token {gist_token}",
        "Accept": "application/vnd.github.v3+json",
    }

    if processed == current_localizations:
        print("No changes to the localizations")
        return 0
    
    content = json.dumps(
        {"localizations": list(processed.values()), "format_version": 1},
        indent=2,
        ensure_ascii=False,
    )

    response = requests.patch(
        f"https://api.github.com/gists/{gist_id}",
        headers=headers,
        json={"files": {"localizations.json": {"content": content}}},
    )

    if response.status_code != 200:
        print(f"Failed to update gist: {response.status_code} {response.text}")
        return 1

    print("Gist updated successfully")
    return 0


if __name__ == "__main__":
    exit(main())
