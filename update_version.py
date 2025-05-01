import re
from typing import Optional, Match


def get_version_from_cargo_toml(file_path: str) -> str:
    with open(file_path, "r", encoding="utf-8") as file:
        content: str = file.read()

        match: Optional[Match[str]] = re.search(r'version\s*=\s*"([^"]+)"', content)

        assert match is not None

        return match.group(1)


def update_tauri_conf_json(file_path: str, version: str) -> None:
    with open(file_path, "r", encoding="utf-8") as file:
        content: str = file.read()

    content = re.sub(r'"version":\s*"[^"]+"', f'"version": "{version}"', content)

    with open(file_path, "w", encoding="utf-8") as file:
        file.write(content)

    print(f"Updated {file_path} to version {version}")


def update_cargo_toml(file_path: str, version: str) -> None:
    with open(file_path, "r", encoding="utf-8") as file:
        content: str = file.read()

    content = re.sub(
        r'version\s*=\s*"[^"]+"', f'version = "{version}"', content, count=1
    )

    with open(file_path, "w+", encoding="utf-8") as file:
        file.write(content)

    print(f"Updated {file_path} to version {version}")


def main() -> None:
    cargo_toml_path: str = "src-tauri/Cargo.toml"
    tauri_conf_json_path: str = "src-tauri/tauri.conf.json"

    current_version: str = get_version_from_cargo_toml(cargo_toml_path)
    print(f"Current version: {current_version}")
    new_version: str = input("Enter new version: ")

    update_cargo_toml(cargo_toml_path, new_version)
    update_tauri_conf_json(tauri_conf_json_path, new_version)


if __name__ == "__main__":
    main()
